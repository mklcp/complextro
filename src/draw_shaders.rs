use crate::emit_shaders;
use miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
}

#[repr(C)]
struct Uniforms {
    transform: [f32; 16],
    resolution: (f32, f32),
}

pub struct Scene {
    pipeline: Pipeline,
    bindings: Bindings,
    center: (f32, f32),
}

impl Scene {
    pub fn new(ctx: &mut Context, fragment_shader: &str) -> Self {
        let vertices: [Vertex; 4] = [
            Vertex {
                pos: Vec2 { x: -1.0, y: -1.0 },
            },
            Vertex {
                pos: Vec2 { x: 1.0, y: -1.0 },
            },
            Vertex {
                pos: Vec2 { x: 1.0, y: 1.0 },
            },
            Vertex {
                pos: Vec2 { x: -1.0, y: 1.0 },
            },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: Vec::new(),
        };

        let shader = Shader::new(
            ctx,
            emit_shaders::VERTEX,
            fragment_shader,
            emit_shaders::meta(),
        )
        .unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[VertexAttribute::new("pos", VertexFormat::Float2)],
            shader,
        );

        Scene {
            pipeline,
            bindings,
            center: (0.0, 0.0),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        // Compute the screen ratio.
        let screen_size = ctx.screen_size();
        let ratio = screen_size.1 / screen_size.0;
        let (scale_x, scale_y) = if ratio <= 1.0 {
            (ratio, 1.0)
        } else {
            (1.0, 1.0 / ratio)
        };

        // Scale and center via an uniform.
        #[rustfmt::skip]
        ctx.apply_uniforms(&Uniforms {
            transform: [
                scale_x, 0.0, 0.0, 0.0,
                0.0, scale_y, 0.0, 0.0,
                0.0,     0.0, 1.0, 0.0,
                (scale_x * self.center.0), (scale_y * self.center.1), 0.0, 1.0,
            ],
            resolution: ctx.screen_size(),
        });

        // Draw the 6 indices of the 4 vertices.
        ctx.draw(0, 2 * 3, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}
