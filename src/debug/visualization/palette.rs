// Color palette management
// src/debug/visualization/palette.rs
pub struct ColorPalette {
    name: String,
    colors: Vec<Color>,
    category: PaletteCategory,
}

#[derive(Debug, Clone, Copy)]
pub enum PaletteCategory {
    Sequential,    // For ordered data
    Diverging,     // For data diverging from a center point
    Qualitative,   // For categorical data
    Alert,         // For status/warning indicators
    Custom,
}

pub struct PaletteGenerator {
    base_hue: f32,
    saturation_range: (f32, f32),
    lightness_range: (f32, f32),
    contrast_ratio: f32,
}

impl PaletteGenerator {
    pub fn generate_sequential(&self, steps: usize) -> ColorPalette {
        // Generate sequential colors with consistent perceptual steps
    }

    pub fn generate_diverging(&self, steps: usize) -> ColorPalette {
        // Generate diverging colors from center point
    }

    pub fn generate_qualitative(&self, count: usize) -> ColorPalette {
        // Generate distinct colors with maximum perceptual difference
    }
}