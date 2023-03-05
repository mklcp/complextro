use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::space0,
    combinator::{fail, map, opt, success},
    multi::many0,
    number::complete::float,
    sequence::{delimited, pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(char, Box<Expr>, Box<Expr>),
    Func(Name, Box<Expr>),
    Real(f32),
    Imaginary(f32),
    VarZ,
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Name {
    Log,
    Neg,
    Pos,
}

type Parse<'a> = IResult<&'a str, Expr>;

//
fn parens(i: &str) -> Parse {
    delimited(space0, delimited(char('('), expression, char(')')), space0)(i)
}

//
fn real(i: &str) -> Parse {
    map(delimited(space0, float, space0), Expr::Real)(i)
}

//
fn imaginary(i: &str) -> Parse {
    delimited(
        space0,
        alt((
            map(terminated(float, char('i')), Expr::Imaginary),
            map(char('i'), |_| Expr::Imaginary(1.0)),
        )),
        space0,
    )(i)
}

// A number is either real or imaginary.
fn number(i: &str) -> Parse {
    alt((imaginary, real))(i)
}

// Only the variable 'z' is supported for now.
fn variable(i: &str) -> Parse {
    map(delimited(space0, char('z'), space0), |_| Expr::VarZ)(i)
}

// A terminal is either a number or a variable.
fn terminal(i: &str) -> Parse {
    alt((number, variable))(i)
}

// Only log is supported for now.
fn fn_call(i: &str) -> Parse {
    map(
        pair(tag("log"), delimited(char('('), expression, char(')'))),
        |(_, e)| Expr::Func(Name::Log, Box::new(e)),
    )(i)
}

//
fn atomic_or_recurse(i: &str) -> Parse {
    alt((terminal, parens, fn_call))(i)
}

// Inside a power, there is either a terminal, a parenthesized expression, or a function call.
fn unary(i: &str) -> Parse {
    map(
        pair(opt(alt((char('-'), char('+')))), atomic_or_recurse),
        |(sop, e)| match sop {
            Some('-') => Expr::Func(Name::Neg, Box::new(e)),
            Some('+') => Expr::Func(Name::Pos, Box::new(e)),
            Some(_) => unreachable!(),
            None => e,
        },
    )(i)
}

fn power(i: &str) -> Parse {
    unary(i)
}

// A factor is made of powers.
fn factor(i: &str) -> Parse {
    let (i, init) = power(i)?;

    let (i, tail) = many0(pair(char('^'), factor))(i)?;

    let mut acc = init;
    for (op_val, factor_val) in tail.into_iter() {
        acc = Expr::Binary(op_val, Box::new(acc), Box::new(factor_val));
    }

    success(acc)(i)
}

// A term is made of factors.
fn term(i: &str) -> Parse {
    let (i, init) = factor(i)?;

    let (i, tail) = many0(pair(alt((char('*'), char('/'))), factor))(i)?;

    let mut acc = init;
    for (op_val, factor_val) in tail.into_iter() {
        acc = Expr::Binary(op_val, Box::new(acc), Box::new(factor_val));
    }

    success(acc)(i)
}

// An expression is made of terms.
fn expression(i: &str) -> Parse {
    let (i, init) = term(i)?;

    let (i, tail) = many0(pair(alt((char('+'), char('-'))), term))(i)?;

    let mut acc = init;
    for (op_val, term_val) in tail.into_iter() {
        acc = Expr::Binary(op_val, Box::new(acc), Box::new(term_val));
    }

    success(acc)(i)

    // The following code will not work because of
    // https://stackoverflow.com/questions/63378620/fold-is-picky-about-closures-it-accepts
    //
    // let e = tail.into_iter().fold(init, |acc, (o, val)| { Expr::Binary(o, Box::new(acc), Box::new(val)) });
    // success(e)(i);

    // The following code will not work because of
    // https://github.com/rust-bakery/nom/issues/255
    // https://github.com/rust-bakery/nom/issues/898
    // fold_many0(
    //   pair(alt((char('+'), char('-'))), term),
    //   || init,
    //   |acc, (o, val): (char, Expr)| { Expr::Binary(o, Box::new(acc), Box::new(val)) },
    // )(i)
}

pub fn parse(i: &str) -> Parse {
    let (i, all_or_empty) = map(opt(expression), |opt_e| match opt_e {
        Some(e) => e,
        None => Expr::Empty,
    })(i)?;
    if i != "" {
        // `expr` must have succeeded, or it was empty.
        // If the remaining of the input is not empty, that means it contains
        // unrecognized characters, so the parser fails here.
        return fail(i);
    }
    Ok((&"", all_or_empty))
}
