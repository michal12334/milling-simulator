use glium::glutin::surface::WindowSurface;
use glium::{uniform, Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use nalgebra::Matrix4;

use crate::vertex::Vertex;


pub struct BlockDrawer {
    program: Program,
}

impl BlockDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 410 core
    
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 view;
    
            void main() {
                gl_Position = perspective * view * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 410 core

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self { program }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        vertex_buffer: &VertexBuffer<Vertex>,
        perspective: &Matrix4<f32>,
        view_matrix: &Matrix4<f32>,
        drawing_parameters: &DrawParameters,
    ) {
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target
            .draw(
                vertex_buffer,
                index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data.0,
                    view: view_matrix.data.0,
                    // obj_color: color,
                    // selected_index: if let Some(x) = selected_index { x as i32 } else { -1 },
                    // selected_color: selected_color,
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
