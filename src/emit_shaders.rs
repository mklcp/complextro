use crate::parser::*;
use miniquad::*;

pub fn ast_to_shader(ast: Expr) -> String {
    match ast {
        Expr::Binary(op, rhs, lhs) => match op {
            '+' => String::from(format!(
                "{}({},{})",
                "add_",
                ast_to_shader(*rhs),
                ast_to_shader(*lhs)
            )),
            '-' => String::from(format!(
                "{}({},{})",
                "sub_",
                ast_to_shader(*rhs),
                ast_to_shader(*lhs)
            )),
            '*' => String::from(format!(
                "{}({},{})",
                "mul_",
                ast_to_shader(*rhs),
                ast_to_shader(*lhs)
            )),
            '/' => String::from(format!(
                "{}({},{})",
                "div_",
                ast_to_shader(*rhs),
                ast_to_shader(*lhs)
            )),
            '^' => String::from(format!(
                "{}({},{})",
                "pow_",
                ast_to_shader(*rhs),
                ast_to_shader(*lhs)
            )),
            _ => unreachable!(),
        },
        Expr::Func(func, e) => match func {
            Name::Log => format!("log_({})", ast_to_shader(*e)),
            Name::Neg => format!("-({})", ast_to_shader(*e)),
            Name::Pos => format!("({})", ast_to_shader(*e)),
        },
        Expr::Real(p) => format!("polar(vec2({:.1}, 0))", p),
        Expr::Imaginary(p) => format!("polar(vec2(0, {:.1}))", p),
        Expr::VarZ => String::from("z"),
        Expr::Empty => String::from("null()"),
    }
}

pub fn build_fragment(s: &str) -> String {
    format!("{}{}{}", FRAGMENT_TEMPLATE_BEGIN, s, FRAGMENT_TEMPLATE_END)
}

const FRAGMENT_TEMPLATE_BEGIN: &str = r#"
#version 100

precision highp float;

varying highp vec2 xy;

//  Function from Iñigo Quiles
//  https://www.shadertoy.com/view/MsS3Wc
vec3 hsb2rgb( in vec3 c ){
    vec3 rgb = clamp(abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),
                             6.0)-3.0)-1.0,
                     0.0,
                     1.0 );
    rgb = rgb*rgb*(3.0-2.0*rgb);
    return c.z * mix( vec3(1.0), rgb, c.y);
}

vec2 cartesian(vec2 z) {
    return z.x * vec2(cos(z.y), sin(z.y));
}

vec2 polar(vec2 z) {
    return vec2(length(z), atan(z.y,z.x));
}

vec2 add_(vec2 z1, vec2 z2) {
    return polar(cartesian(z1) + cartesian(z2));
}

vec2 sub_(vec2 z1, vec2 z2) {
    return polar(cartesian(z1) - cartesian(z2));
}

vec2 mul_(vec2 z1, vec2 z2) {
    return vec2(z1.x*z2.x, z1.y+z2.y);
}

vec2 div_(vec2 z1, vec2 z2) {
    return vec2(z1.x/z2.x, z1.y-z2.y);
}

vec2 log_(vec2 z) {
    return polar(vec2(log(z.x), z.y));
}

vec2 pow_(float n, vec2 z) {
    return vec2(exp(log(n)*z.x*cos(z.y)), log(n)*z.x*sin(z.y));
}

vec2 pow_(vec2 z1, vec2 z2) {
    return pow_(exp(1.), mul_(log_(z1),z2));
}

vec2 pow_(vec2 z, float n) {
    return vec2(exp(log(z.x)*n), z.y*n);
}

vec2 null() {
    return vec2(0.,0.);
}

vec2 id(vec2 z) {
    return z;
}

vec2 compact(vec2 z) {
    float rho = z.s;
    float theta = z.t;
    return vec2(log(abs(rho))/(1. + abs(log(abs(rho)))) + 0.5, theta);

}

#define TWO_PI 6.28318530718

vec3 color(vec2 z) {
    float h = z.t/TWO_PI + 0.5;
    float s = 1.;
    float b = z.s*0.7;
    return hsb2rgb(vec3(h, s, b));
}

void main(void) {
    vec2 z = polar(xy*4.); 

    gl_FragColor = vec4( color( compact( "#;

const FRAGMENT_TEMPLATE_END: &str = r#"
 ) ), 1 );
}
"#;

pub const VERTEX: &str = r#"

#version 100

uniform highp mat4 transform;

attribute highp vec2 pos;
varying highp vec2 xy;

void main() {
    gl_Position = transform*vec4(pos, 0, 1);
    xy = pos;
}

"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("transform", UniformType::Mat4)],
        },
    }
}
