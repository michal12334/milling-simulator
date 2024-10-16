use glium::glutin::surface::WindowSurface;
use glium::{uniform, BackfaceCullingMode, Display, DrawParameters, Frame, PolygonMode, Program, Surface, VertexBuffer};
use nalgebra::Matrix4;

use crate::vertex::SmallVertex;


pub struct GCodeDrawer {
    program: Program,
}

impl GCodeDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 410 core
    
            in vec3 position;

            uniform mat4 perspective;
            uniform mat4 view;

            void main() {
                gl_Position = perspective * view * vec4(position / 10.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 410 core

            out vec4 frag_color;

            const vec3 color = vec3(0.0, 0.8, 0.0);

            void main() {
                frag_color = vec4(color, 1.0);
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self { program }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        vertex_buffer: &VertexBuffer<SmallVertex>,
        perspective: &Matrix4<f32>,
        view_matrix: &Matrix4<f32>,
        drawing_parameters: &DrawParameters,
    ) {
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
        let mut drawing_parameters = drawing_parameters.clone();
        drawing_parameters.polygon_mode = PolygonMode::Line;
        drawing_parameters.backface_culling = BackfaceCullingMode::CullingDisabled;

        target
            .draw(
                vertex_buffer,
                index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data.0,
                    view: view_matrix.data.0,
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
