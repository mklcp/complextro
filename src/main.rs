mod draw_shaders;
mod emit_shaders;
mod gui;
mod parser;

fn main() {
    let conf = miniquad::conf::Conf {
        high_dpi: true,
        window_width: 1200,
        window_height: 1024,
        window_title: "Complextro".to_string(),
        ..Default::default()
    };
    miniquad::start(conf, |mut ctx| Box::new(gui::Interface::new(&mut ctx)));
}
