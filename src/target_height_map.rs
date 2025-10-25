use derive_getters::Getters;
use glium::{glutin::surface::WindowSurface, Display, Rect, Texture2d};
use serde::{Deserialize, Serialize};

#[derive(Debug, Getters, Serialize, Deserialize)]
pub struct TargetHeightMap {
    heights: Vec<Vec<f32>>,
}

impl Default for TargetHeightMap {
    fn default() -> Self {
        Self {
            heights: vec![vec![0.0; 100]; 100],
        }
    }
}

impl TargetHeightMap {
    pub fn to_texture(&self, display: &Display<WindowSurface>) -> Texture2d {
        let texture = Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            self.heights[0].len() as u32,
            self.heights.len() as u32,
        )
        .unwrap();

        texture.write(
            Rect {
                left: 0,
                bottom: 0,
                width: self.heights[0].len() as u32,
                height: self.heights.len() as u32,
            },
            self.heights
                .iter()
                .map(|x| x.iter().map(|y| y / 10.0).collect())
                .collect::<Vec<Vec<_>>>(),
        );

        texture
    }
}
