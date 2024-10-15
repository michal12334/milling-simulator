use derive_getters::Getters;

use crate::g_code_instruction::GCodeInstruction;

#[derive(Debug, Clone, Getters)]
pub struct GCode {
    instructions: Vec<GCodeInstruction>,
}

impl GCode {
    pub fn from_file(file_path: &str) -> Self {
        let content = std::fs::read_to_string(file_path).unwrap();
        let instructions = content.split_whitespace()
            .filter_map(|line| { GCodeInstruction::parse(line) })
            .collect();
        Self { instructions }
    }
}
