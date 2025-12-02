//! Format menu (font)
//!
//! This module handles format settings including font selection.

/// Format settings including font preferences
#[derive(Default)]
pub struct FormatSettings {
    /// Font family name
    pub font_family: String,
    /// Font size in points
    pub font_size: f32,
}

impl FormatSettings {}
