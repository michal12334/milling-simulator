use derive_getters::Getters;
use glium::implement_vertex;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: u8,
}

implement_vertex!(Vertex, position, tex_coords, normal);

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2], normal: u8) -> Self {
        Self {
            position,
            tex_coords,
            normal,
        }
    }

    pub fn from_tuples(position: (f32, f32, f32), tex_coords: (f32, f32), normal: u8) -> Self {
        Self::new([position.0, position.1, position.2], [tex_coords.0, tex_coords.1], normal)
    }
}

#[derive(Debug, Copy, Clone, Default, Getters)]
pub struct SmallVertex {
    position: [f32; 3],
}

implement_vertex!(SmallVertex, position);

impl SmallVertex {
    pub fn new(position: [f32; 3]) -> Self {
        Self { position, }
    }
}
