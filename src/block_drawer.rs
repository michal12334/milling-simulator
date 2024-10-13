use glium::glutin::surface::WindowSurface;
use glium::{uniform, Display, DrawParameters, Frame, Program, Surface, Texture2d, VertexBuffer};
use nalgebra::{Matrix4, Vector3};

use crate::vertex::Vertex;


pub struct BlockDrawer {
    program: Program,
}

impl BlockDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 410 core
    
            in vec3 position;
            in int normal;
            in vec2 tex_coords;

            out vec3 normal_out;
            out vec3 world;

            uniform sampler2D height_map;
            
            uniform mat4 perspective;
            uniform mat4 view;
    
            void main() {
                float height = position.y;

                if (height > 0) {
                    height = texture(height_map, tex_coords).x / 2.0;
                }

                world = vec3(position.x, height, position.z);
                gl_Position = perspective * view * vec4(world, 1.0);

                if (normal == 0) {
                    normal_out = vec3(0.0, 1.0, 0.0);
                } else if (normal == 1) {
                    normal_out = vec3(0.0, -1.0, 0.0);
                } else if (normal == 2) {
                    normal_out = vec3(1.0, 0.0, 0.0);
                } else if (normal == 3) {
                    normal_out = vec3(-1.0, 0.0, 0.0);
                } else if (normal == 4) {
                    normal_out = vec3(0.0, 0.0, 1.0);
                } else if (normal == 5) {
                    normal_out = vec3(0.0, 0.0, -1.0);
                }
            }
        "#;

        let fragment_shader_src = r#"
            #version 410 core

            in vec3 normal_out;
            in vec3 world;

            out vec4 frag_color;

            const vec3 light_pos = vec3(7.0, 25.0, -7.0);
            const vec3 color = vec3(0.8, 0.8, 0.8);

            uniform vec3 cam_pos;

            void main() {
                vec3 to_cam = normalize(cam_pos - world);
                vec3 to_light = normalize(light_pos - world);

                float ambient = 0.3;
                float diffuse =  max(dot(normal_out, to_light), 0.0);
                vec3 reflected = normalize(reflect(-to_light, normal_out));
                float specular = pow(max(dot(reflected, to_cam), 0.0), 50.0);

                frag_color = vec4((ambient + diffuse + specular) * color, 1.0);
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
        camera_position: Vector3<f32>,
        height_map: &Texture2d,
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
                    cam_pos: camera_position.data.0[0],
                    height_map: height_map.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
