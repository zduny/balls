use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    name: String,
    rgb: (u8, u8, u8),
}

impl Color {
    pub fn new(name: &str, red: u8, green: u8, blue: u8) -> Self {
        Color {
            name: name.to_string(),
            rgb: (red, green, blue),
        }
    }

    pub fn into_style_value(&self) -> String {
        let rgb = self.rgb;
        format!("rgb({}, {}, {})", rgb.0, rgb.1, rgb.2)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
