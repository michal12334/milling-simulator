use std::collections::HashSet;

use glium::{glutin::surface::WindowSurface, Display, Rect, Texture2d};

pub struct HeightMap {
    texture: Texture2d,
    data: Vec<Vec<f32>>,
    changed_indices: HashSet<(usize, usize)>,
}

impl HeightMap {
    pub fn new(resolution: (u32, u32, u32), height: f32, display: &Display<WindowSurface>) -> Self {
        let texture = Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            resolution.0,
            resolution.2,
        )
        .unwrap();

        let data = vec![vec![height; resolution.0 as usize]; resolution.2 as usize];

        texture.write(
            Rect {
                left: 0,
                bottom: 0,
                width: resolution.0,
                height: resolution.2,
            },
            data.clone(),
        );

        Self {
            texture,
            data,
            changed_indices: HashSet::new(),
        }
    }

    pub fn get_texture(&self) -> &Texture2d {
        &self.texture
    }

    pub fn write(&mut self, index: (usize, usize), height: f32) {
        self.data[index.0][index.1] = height;
        self.changed_indices.insert((index.0, index.1));
    }

    pub fn update_texture(&mut self) {
        for i in self.changed_indices.iter() {
            self.texture.write(
                Rect {
                    left: i.0 as u32,
                    bottom: i.1 as u32,
                    width: 1,
                    height: 1,
                },
                vec![vec![self.data[i.0][i.1]]],
            );
        }
        self.changed_indices.clear();
    }

    pub fn get_height(&self, index: (usize, usize)) -> f32 {
        self.data[index.0][index.1]
    }
}
