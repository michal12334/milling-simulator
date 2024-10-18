use glium::glutin::surface::WindowSurface;
use glium::{uniform, Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use nalgebra::{Matrix4, Vector3};

use crate::vertex::SmallVertex;

pub struct GCodeExecutorDrawer {
    program: Program,
    vertex_buffer: VertexBuffer<SmallVertex>,
}

impl GCodeExecutorDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 410 core
    
            in vec3 position;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 410 core

            out vec4 frag_color;

            const vec3 color = vec3(0.8, 0.0, 0.0);

            void main() {
                frag_color = vec4(color, 1.0);
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let size = 0.4;
        let y_size = 2.0;

        let vertex_buffer = VertexBuffer::new(
            display,
            &[
                SmallVertex::new([-size, 0.0, -size]),
                SmallVertex::new([size, 0.0, -size]),
                SmallVertex::new([-size, 0.0, size]),
                SmallVertex::new([size, 0.0, -size]),
                SmallVertex::new([size, 0.0, size]),
                SmallVertex::new([-size, 0.0, size]),
                SmallVertex::new([-size, y_size, -size]),
                SmallVertex::new([-size, y_size, size]),
                SmallVertex::new([size, y_size, -size]),
                SmallVertex::new([size, y_size, -size]),
                SmallVertex::new([-size, y_size, size]),
                SmallVertex::new([size, y_size, size]),
                SmallVertex::new([-size, 0.0, -size]),
                SmallVertex::new([-size, y_size, -size]),
                SmallVertex::new([size, 0.0, -size]),
                SmallVertex::new([size, 0.0, -size]),
                SmallVertex::new([-size, y_size, -size]),
                SmallVertex::new([size, y_size, -size]),
                SmallVertex::new([-size, 0.0, size]),
                SmallVertex::new([size, 0.0, size]),
                SmallVertex::new([-size, y_size, size]),
                SmallVertex::new([size, 0.0, size]),
                SmallVertex::new([size, y_size, size]),
                SmallVertex::new([-size, y_size, size]),
                SmallVertex::new([-size, 0.0, size]),
                SmallVertex::new([-size, y_size, -size]),
                SmallVertex::new([-size, 0.0, -size]),
                SmallVertex::new([-size, 0.0, size]),
                SmallVertex::new([-size, y_size, size]),
                SmallVertex::new([-size, y_size, -size]),
                SmallVertex::new([size, 0.0, size]),
                SmallVertex::new([size, 0.0, -size]),
                SmallVertex::new([size, y_size, -size]),
                SmallVertex::new([size, 0.0, size]),
                SmallVertex::new([size, y_size, -size]),
                SmallVertex::new([size, y_size, size]),
            ],
        )
        .unwrap();

        Self {
            program,
            vertex_buffer,
        }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        perspective: &Matrix4<f32>,
        view_matrix: &Matrix4<f32>,
        position: (f32, f32, f32),
        drawing_parameters: &DrawParameters,
    ) {
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let model = Matrix4::new_translation(&Vector3::new(position.0, position.1, position.2));

        target
            .draw(
                &self.vertex_buffer,
                index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data.0,
                    view: view_matrix.data.0,
                    model: model.data.0,
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
