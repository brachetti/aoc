use crate::lib::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone, Deserialize, Eq, Hash)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
}

// taken from https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let input = hex_code.replace("#", "");
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&input[0..2], 16)?;
        let g: u8 = u8::from_str_radix(&input[2..4], 16)?;
        let b: u8 = u8::from_str_radix(&input[4..6], 16)?;

        Ok(RGB { r, g, b })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_with_hash() {
        let hex = "#00FF00";
        let converted = RGB::from_str(hex).unwrap();
        assert_eq!(converted, RGB { r: 0, g: 255, b: 0 })
    }

    #[test]
    fn should_convert_without_hash() {
        let hex = "FFFF00";
        let converted = RGB::from_str(hex).unwrap();
        assert_eq!(
            converted,
            RGB {
                r: 255,
                g: 255,
                b: 0,
            }
        )
    }
}
