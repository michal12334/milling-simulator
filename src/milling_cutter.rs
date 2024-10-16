#[derive(Debug, Clone)]
pub enum MillingCutter {
    Flat(u8),
    Spherical(u8),
}

impl MillingCutter {
    pub fn parse(file_extension: &str) -> Option<Self> {
        let size = file_extension[1..].parse::<u8>().ok()?;

        match file_extension.chars().nth(0).unwrap() {
            'k' => Some(MillingCutter::Spherical(size)),
            'f' => Some(MillingCutter::Flat(size)),
            _ => None,
        }
    }
}
