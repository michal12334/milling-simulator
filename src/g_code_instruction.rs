use derive_getters::Getters;

#[derive(Debug, Clone, Getters)]
pub struct GCodeInstruction {
    #[getter(copy)]
    n: u32,
    #[getter(copy)]
    x: Option<f32>,
    #[getter(copy)]
    y: Option<f32>,
    #[getter(copy)]
    z: Option<f32>,
}

impl GCodeInstruction {
    pub fn parse(line: &str) -> Option<GCodeInstruction> {
        let instruction = line.trim();
        let n_begin = instruction.find("N")?;
        let g_begin = instruction.find("G")?;
        let n = instruction[(n_begin + 1)..g_begin].parse::<u32>().ok()?;

        let x_begin = instruction.find("X");
        let y_begin = instruction.find("Y");
        let z_begin = instruction.find("Z");

        let mut x = None;
        let mut y = None;
        let mut z = None;

        if let Some(x_begin) = x_begin {
            let x_end = if y_begin.is_some() {
                    y_begin
                } else if z_begin.is_some() {
                    z_begin
                } else {
                    None
                };
            
            let x_as_str = match x_end {
                Some(x_end) => &instruction[(x_begin + 1)..x_end],
                None => &instruction[(x_begin + 1)..],
            };

            x = Some(x_as_str.parse::<f32>().ok()?);
        }

        if let Some(y_begin) = y_begin {
            let y_end = z_begin;
            
            let y_as_str = match y_end {
                Some(y_end) => &instruction[(y_begin + 1)..y_end],
                None => &instruction[(y_begin + 1)..],
            };

            y = Some(y_as_str.parse::<f32>().ok()?);
        }

        if let Some(z_begin) = z_begin {
            let z_as_str = &instruction[(z_begin + 1)..];

            z = Some(z_as_str.parse::<f32>().ok()?);
        }

        Some(GCodeInstruction {
            n, x, y, z,
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::GCodeInstruction;

    #[rstest]
    #[case("N1G01", 1, None, None, None)]
    #[case("  \n  N145G01   ", 145, None, None, None)]
    #[case("N123G01X00.000", 123, Some(0.0), None, None)]
    #[case("N123G01X00.000Y00.000", 123, Some(0.0), Some(0.0), None)]
    #[case("N123G01X00.000Y00.000Z00.000", 123, Some(0.0), Some(0.0), Some(0.0))]
    #[case("N123G01X12.345Y21.555Z05.005", 123, Some(12.345), Some(21.555), Some(5.005))]
    #[case("N123G01X-12.345Y21.555Z-05.005", 123, Some(-12.345), Some(21.555), Some(-5.005))]
    fn code_is_parsed(#[case] line: &str, #[case] n: u32, #[case] x: Option<f32>, #[case] y: Option<f32>, #[case] z: Option<f32>) {
        let instruction = GCodeInstruction::parse(line);
        assert!(instruction.is_some());

        let instruction = instruction.unwrap();

        assert_eq!(instruction.n(), n);
        assert_eq!(instruction.x(), x);
        assert_eq!(instruction.y(), y);
        assert_eq!(instruction.z(), z);
    }

    #[rstest]
    #[case("N1aG01")]
    #[case("")]
    #[case("N12301X00.000Y00.000Z00.000")]
    #[case("N123G01Xa00.000Y00b.000Z00.000")]
    fn code_cannot_be_parsed(#[case] line: &str) {
        let instruction = GCodeInstruction::parse(line);
        assert!(instruction.is_none());
    }
}
