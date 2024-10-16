use derive_getters::Getters;
use derive_new::new;

use crate::{g_code::GCode, milling_cutter::MillingCutter};

#[derive(Debug, Clone, Getters)]
pub struct GCodeExecutor {
    current_instruction: usize,
    current_position: (f32, f32, f32),
    cutter: Vec<CutterPart>,
    code: GCode,
    resolution: (u32, u32),
    size: (f32, f32, f32),
}

#[derive(Debug, Clone, Getters, new)]
pub struct CutterPart {
    index_offset: (usize, usize),
    position_offset: (f32, f32, f32),
}

impl GCodeExecutor {
    pub fn new(code: GCode, resolution: (u32, u32), size: (f32, f32, f32)) -> Self {
        let cutter = Self::get_cutter(&code, resolution, size);

        Self {
            current_position: (0.0, 30.0, 0.0),
            current_instruction: 0,
            code,
            cutter,
            resolution,
            size,
        }
    }

    pub fn load(&mut self, code: GCode) {
        self.current_instruction = 0;
        self.cutter = Self::get_cutter(&code, self.resolution, self.size);
        self.code = code;
    }

    fn get_cutter(code: &GCode, resolution: (u32, u32), size: (f32, f32, f32)) -> Vec<CutterPart> {
        let cutter_size = match code.cutter() {
            MillingCutter::Flat(size) => *size,
            MillingCutter::Spherical(size) => *size,
        } as f32 / 10.0;

        let single_size = (size.0 / (resolution.0 as f32), size.2 / (resolution.1 as f32));

        let cutter_space = ((cutter_size / single_size.0) as usize, (cutter_size / single_size.1) as usize);

        (0..cutter_space.0)
            .flat_map(|x| (0..cutter_space.1).map(move |z| (x, z)))
            .map(|(x, z)| {
                let x_offset = single_size.0 * x as f32;
                let z_offset = single_size.1 * z as f32;
                let y_offset = match code.cutter() {
                    MillingCutter::Flat(_) => 0.0,
                    MillingCutter::Spherical(_) => (cutter_size.powi(2) - x_offset.powi(2) - z_offset.powi(2)).sqrt() + cutter_size,
                };

                CutterPart::new((x, z), (x_offset, y_offset, z_offset))
            })
            .collect()
    }
}
