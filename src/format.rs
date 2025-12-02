//! Format menu (font)
//!
//! This module handles format settings including font selection.

/// Font family options
///
/// Represents the available font families in egui.
/// Monospace fonts are fixed-width, Proportional fonts are variable-width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFamily {
    /// Monospace (fixed-width) font family
    #[default]
    Monospace,
    /// Proportional (variable-width) font family
    Proportional,
}

impl FontFamily {
    /// Get display name for the font family
    ///
    /// # Returns
    /// Human-readable name of the font family
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Monospace => "Monospace",
            Self::Proportional => "Proportional",
        }
    }

    /// Get all available font families
    ///
    /// # Returns
    /// Vector of all font family variants
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![Self::Monospace, Self::Proportional]
    }
}

/// Font style options
///
/// Represents font styling options. Currently supports Regular style.
/// Bold and Italic styles would require loading custom fonts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontStyle {
    /// Regular/normal font style
    #[default]
    Regular,
    /// Bold font style (requires custom font)
    Bold,
    /// Italic font style (requires custom font)
    Italic,
    /// Bold italic font style (requires custom font)
    BoldItalic,
}

impl FontStyle {
    /// Get display name for the font style
    ///
    /// # Returns
    /// Human-readable name of the font style
    #[allow(dead_code)] // Kept for future use or config compatibility
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Regular => "Regular",
            Self::Bold => "Bold",
            Self::Italic => "Italic",
            Self::BoldItalic => "Bold Italic",
        }
    }

    /// Get all available font styles
    ///
    /// # Returns
    /// Vector of all font style variants
    #[allow(dead_code)] // Kept for future use or config compatibility
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![Self::Regular, Self::Bold, Self::Italic, Self::BoldItalic]
    }
}

/// Format settings including font preferences
#[allow(clippy::struct_field_names)] // Font-related fields naturally share prefix
#[derive(Default)]
pub struct FormatSettings {
    /// Font family name (kept for backward compatibility with config)
    pub font_family: String,
    /// Font family selection (Monospace or Proportional)
    pub font_family_type: FontFamily,
    /// Font style (Regular, Bold, Italic, `BoldItalic`)
    pub font_style: FontStyle,
    /// Font size in points
    pub font_size: f32,
}

impl FormatSettings {}
