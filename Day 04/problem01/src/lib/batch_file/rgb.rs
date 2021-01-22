use crate::lib::prelude::*;

use anyhow::bail;

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
    type Err = anyhow::Error;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, anyhow::Error> {
        if !hex_code.starts_with("#") || !hex_code.len() == 7 {
            bail!("Invalid format")
        }

        // assert!()
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(RGB { r, g, b })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_with_hash() {
        let hex = "#FFFF00";
        let converted = RGB::from_str(hex);
        assert_eq!(
            converted.unwrap(),
            RGB {
                r: 255,
                g: 255,
                b: 0,
            }
        )
    }

    #[test]
    fn should_not_convert_without_hash() {
        let hex = "FFFF00";
        let converted = RGB::from_str(hex);
        assert!(converted.is_err())
    }
}
