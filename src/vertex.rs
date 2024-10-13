use glium::implement_vertex;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }

    pub fn from_tuples(position: (f32, f32, f32), tex_coords: (f32, f32)) -> Self {
        Self::new([position.0, position.1, position.2], [tex_coords.0, tex_coords.1])
    }
}
