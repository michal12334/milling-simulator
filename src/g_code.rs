use derive_getters::Getters;

use crate::{g_code_instruction::GCodeInstruction, milling_cutter::MillingCutter};

#[derive(Debug, Clone, Getters)]
pub struct GCode {
    instructions: Vec<GCodeInstruction>,
    cutter: MillingCutter,
}

impl GCode {
    pub fn from_file(file_path: &str) -> Option<Self> {
        let dot_position = file_path.find(".")?;
        let file_extension = &file_path[(dot_position+1)..];
        let cutter = MillingCutter::parse(file_extension)?;

        let content = std::fs::read_to_string(file_path).unwrap();
        let instructions = content.split_whitespace()
            .filter_map(|line| { GCodeInstruction::parse(line) })
            .collect();
        Some(Self { instructions, cutter, })
    }
}
