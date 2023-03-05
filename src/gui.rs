use egui;
use {egui_miniquad as egui_mq, miniquad as mq};

use crate::draw_shaders;
use crate::emit_shaders;
use crate::parser;

pub struct Interface {
    egui_mq: egui_mq::EguiMq,
    text: String,
    shader_expr: String,
    parse_msg: Option<String>,
    scene: draw_shaders::Scene,
}

impl Interface {
    pub fn new(ctx: &mut mq::Context) -> Self {
        let null_shader = "null()".to_string();
        Self {
            scene: draw_shaders::Scene::new(ctx, &emit_shaders::build_fragment(&null_shader)),
            text: "".to_string(),
            egui_mq: egui_mq::EguiMq::new(ctx),
            shader_expr: null_shader,
            parse_msg: Some(DEFAULT_MSG.to_string()),
        }
    }
}

static DEFAULT_MSG: &'static str = "Type an expression of the complex variable z, then hit Enter";

const EXAMPLES: [&str; 13] = [
    "z",
    "z^2 + 3",
    "log(z)",
    "z^70000",
    "z^1000",
    "z^100",
    "z^z*1000",
    "z^(0.1-log(z))",
    "z/(1+z^10)",
    "(z^2 + z)/(z^-2 - z)",
    "z^z^z^z",
    "z^log(z)^(1/z)",
    "z^z^14",
];

fn parse_input(text: &String, shader_expr: &mut String, parse_msg: &mut Option<String>) {
    match parser::parse(text) {
        Ok((_, parse_output)) => {
            *shader_expr = emit_shaders::ast_to_shader(parse_output);
            *parse_msg = Some(DEFAULT_MSG.to_string());
        }
        Err(err) => {
            *shader_expr = "null()".to_string();
            *parse_msg = Some(format!("{:?}", err).to_string());
        }
    };
}

impl mq::EventHandler for Interface {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, mq_ctx: &mut mq::Context) {
        mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        mq_ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        mq_ctx.end_render_pass();

        // UI
        self.egui_mq.run(mq_ctx, |_mq_ctx, egui_ctx| {
            egui_ctx.set_pixels_per_point(1.7);

            egui::Window::new("Input").show(egui_ctx, |ui| {
                let complex_expr_input = ui.add(
                    egui::TextEdit::singleline(&mut self.text)
                        .lock_focus(true)
                        .hint_text("e.g.z^2 + 3"),
                );
                if complex_expr_input.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    parse_input(&self.text, &mut self.shader_expr, &mut self.parse_msg);
                }

                ui.label(format!(
                    "{}",
                    match &self.parse_msg {
                        Some(msg) => &msg,
                        None => "",
                    }
                ));
            });

            egui::Window::new("Examples").show(egui_ctx, |ui| {
                for (c, ex) in EXAMPLES.iter().enumerate() {
                    if ui.button(*ex).clicked() {
                        self.text = EXAMPLES[c].to_string();
                        parse_input(&self.text, &mut self.shader_expr, &mut self.parse_msg);
                    }
                }
            });
        });

        let shader = &emit_shaders::build_fragment(&self.shader_expr);
        self.scene = draw_shaders::Scene::new(mq_ctx, shader);
        self.scene.draw(mq_ctx);

        self.egui_mq.draw(mq_ctx);

        mq_ctx.commit_frame();
    }

    // Code that takes care of the interop between user inputs and Egui/Miniquad

    fn mouse_motion_event(&mut self, _: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}
