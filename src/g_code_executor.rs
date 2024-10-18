use derive_getters::Getters;
use derive_new::new;
use line_drawing::Bresenham3d;

use crate::{g_code::GCode, height_map::HeightMap, milling_cutter::MillingCutter};

#[derive(Debug, Clone, Getters)]
pub struct GCodeExecutor {
    current_instruction: usize,
    current_position: (f32, f32, f32),
    cutter: Vec<CutterPart>,
    code: GCode,
    resolution: (u32, u32, u32),
    size: (f32, f32, f32),
    current_points: Option<Vec<(i32, i32, i32)>>,
    current_point: Option<usize>,
}

#[derive(Debug, Clone, Getters, new)]
pub struct CutterPart {
    index_offset: (usize, usize),
    position_offset: (f32, f32, f32),
}

impl GCodeExecutor {
    pub fn new(code: GCode, resolution: (u32, u32, u32), size: (f32, f32, f32)) -> Self {
        let cutter = Self::get_cutter(&code, resolution, size);

        Self {
            current_position: (0.0, 22.0, 0.0),
            current_instruction: 0,
            code,
            cutter,
            resolution,
            size,
            current_points: None,
            current_point: None,
        }
    }

    pub fn load(&mut self, code: GCode) {
        self.current_instruction = 0;
        self.cutter = Self::get_cutter(&code, self.resolution, self.size);
        self.code = code;
        self.current_point = None;
        self.current_points = None;
    }

    fn get_cutter(
        code: &GCode,
        resolution: (u32, u32, u32),
        size: (f32, f32, f32),
    ) -> Vec<CutterPart> {
        let cutter_size = match code.cutter() {
            MillingCutter::Flat(size) => *size,
            MillingCutter::Spherical(size) => *size,
        } as f32
            / 20.0;

        let single_size: (f32, f32) = (
            size.0 / (resolution.0 as f32),
            size.2 / (resolution.2 as f32),
        );

        let cutter_space = (
            (cutter_size / single_size.0) as usize,
            (cutter_size / single_size.1) as usize,
        );

        (0..cutter_space.0)
            .flat_map(|x| (0..cutter_space.1).map(move |z| (x, z)))
            .map(|(x, z)| {
                let x_offset = single_size.0 * x as f32;
                let z_offset = single_size.1 * z as f32;
                let y_offset = match code.cutter() {
                    MillingCutter::Flat(_) => 0.0,
                    MillingCutter::Spherical(_) => {
                        cutter_size
                            - (cutter_size.powi(2) - x_offset.powi(2) - z_offset.powi(2))
                                .max(0.0)
                                .sqrt()
                    }
                };

                CutterPart::new((x, z), (x_offset, y_offset, z_offset))
            })
            .collect()
    }

    pub fn execution_finished(&self) -> bool {
        self.current_instruction >= self.code.instructions().len() - 1
    }

    pub fn execute_step(&mut self, height_map: &mut HeightMap) {
        if self.execution_finished() {
            return;
        }

        let single_size = (
            self.size.0 / (self.resolution.0 as f32),
            self.size.1 / (self.resolution.1 as f32),
            self.size.2 / (self.resolution.2 as f32),
        );
        if self.current_points.is_none() {
            let start = (
                (self.current_position.0 / single_size.0) as i32,
                (self.current_position.1 / single_size.1) as i32,
                (self.current_position.2 / single_size.2) as i32,
            );
            let instruction = self.code.instructions()[self.current_instruction].clone();
            let end_x = match instruction.x() {
                Some(v) => (v / single_size.0 / 10.0) as i32,
                None => start.0,
            };
            let end_y = match instruction.y() {
                Some(v) => (v / single_size.2 / 10.0) as i32,
                None => start.2,
            };
            let end_z = match instruction.z() {
                Some(v) => (v / single_size.1 / 10.0) as i32,
                None => start.1,
            };
            let end = (end_x, end_z, end_y);
            self.current_points = Some(Bresenham3d::new(start, end).collect());
            self.current_point = Some(0);
        }

        let current_point = self.current_point.unwrap();
        let current_points = self.current_points.as_mut().unwrap();

        let point = current_points[current_point];

        self.current_position = (
            single_size.0 * point.0 as f32,
            single_size.1 * point.1 as f32,
            single_size.2 * point.2 as f32,
        );

        for c in self.cutter.iter() {
            let xs = [
                point.0 + self.resolution.0 as i32 / 2 + c.index_offset.0 as i32,
                point.0 + self.resolution.0 as i32 / 2 - c.index_offset.0 as i32,
            ];
            let zs = [
                point.2 + self.resolution.2 as i32 / 2 + c.index_offset.1 as i32,
                point.2 + self.resolution.2 as i32 / 2 - c.index_offset.1 as i32,
            ];
            for (x, z) in xs.iter().flat_map(|x| zs.iter().map(|z| (*x, *z))) {
                if x >= 0 && x < self.resolution.0 as i32 && z >= 0 && z < self.resolution.2 as i32
                {
                    let index = (x as usize, z as usize);
                    if height_map.get_height(index) >= self.current_position.1 + c.position_offset.1
                    {
                        height_map.write(index, self.current_position.1 + c.position_offset.1);
                    }
                }
            }
        }

        if current_point >= current_points.len() - 1 {
            self.current_point = None;
            self.current_points = None;
            self.current_instruction += 1;
        } else {
            self.current_point = Some(current_point + 1);
        }
    }
}
