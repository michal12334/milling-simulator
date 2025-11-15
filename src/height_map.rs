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
            resolution.2,
            resolution.0,
        )
        .unwrap();

        let data = vec![vec![height; resolution.2 as usize]; resolution.0 as usize];

        texture.write(
            Rect {
                left: 0,
                bottom: 0,
                width: resolution.2,
                height: resolution.0,
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
        if self.changed_indices.is_empty() {
            return;
        }

        let left = self.changed_indices.iter().map(|i| i.1).min().unwrap();
        let bottom = self.changed_indices.iter().map(|i| i.0).min().unwrap();
        let width = self.changed_indices.iter().map(|i| i.1).max().unwrap() - left + 1;
        let height = self.changed_indices.iter().map(|i| i.0).max().unwrap() - bottom + 1;

        self.texture.write(
            Rect {
                left: left as u32,
                bottom: bottom as u32,
                width: width as u32,
                height: height as u32,
            },
            self.data[bottom..(bottom + height)]
                .iter()
                .map(|v| v[left..(left + width)].to_vec())
                .collect::<Vec<_>>(),
        );
        self.changed_indices.clear();
    }

    pub fn get_height(&self, index: (usize, usize)) -> f32 {
        self.data[index.0][index.1]
    }
}
