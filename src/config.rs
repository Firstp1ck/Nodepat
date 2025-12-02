//! Configuration file management
//!
//! This module handles loading and saving configuration from config.jsonc
//! including recent files, font settings, and window preferences.

use crate::format::{FontFamily, FontStyle, FormatSettings};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Recent files list
    pub recent_files: Vec<String>,
    /// Font family (kept for backward compatibility)
    pub font_family: String,
    /// Font family type (Monospace or Proportional)
    #[serde(default)]
    pub font_family_type: FontFamily,
    /// Font style (Regular, Bold, Italic, `BoldItalic`)
    #[serde(default)]
    pub font_style: FontStyle,
    /// Font size
    pub font_size: f32,
    /// Status bar visible
    pub show_status_bar: bool,
    /// Dark mode enabled
    pub dark_mode: bool,
    /// Window width
    pub window_width: f32,
    /// Window height
    pub window_height: f32,
}

impl Config {
    /// Load configuration from file
    ///
    /// # Returns
    /// Config struct with loaded values or defaults
    #[must_use]
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if let Ok(content) = fs::read_to_string(&config_path) {
            // Try to parse as JSON
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        Self::create_default()
    }

    /// Create default configuration
    ///
    /// # Returns
    /// Default Config struct
    #[must_use]
    fn create_default() -> Self {
        Self {
            recent_files: Vec::new(),
            font_family: "Courier New".to_string(),
            font_family_type: FontFamily::Monospace,
            font_style: FontStyle::Regular,
            font_size: 10.0,
            show_status_bar: false,
            dark_mode: true,
            window_width: 640.0,
            window_height: 480.0,
        }
    }

    /// Save configuration to file
    ///
    /// # Returns
    /// Result indicating success or error
    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {e}"))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {e}"))?;
        fs::write(&config_path, json).map_err(|e| format!("Failed to write config: {e}"))?;
        Ok(())
    }

    /// Get configuration file path
    ///
    /// # Returns
    /// Path to config.jsonc file
    #[must_use]
    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("Nodepat");
        path.push("config.jsonc");
        path
    }

    /// Add file to recent files list
    ///
    /// # Arguments
    /// * `file_path` - Path to add
    pub fn add_recent_file(&mut self, file_path: &str) {
        // Remove if already exists
        self.recent_files.retain(|f| f != file_path);
        // Add to front
        self.recent_files.insert(0, file_path.to_string());
        // Limit to 10 recent files
        if self.recent_files.len() > 10 {
            self.recent_files.truncate(10);
        }
    }

    /// Apply format settings from config
    ///
    /// # Arguments
    /// * `format_settings` - Format settings to update
    pub fn apply_to_format(&self, format_settings: &mut FormatSettings) {
        format_settings.font_family.clone_from(&self.font_family);
        format_settings.font_family_type = self.font_family_type;
        format_settings.font_style = self.font_style;
        format_settings.font_size = self.font_size;
    }

    /// Update config from format settings
    ///
    /// # Arguments
    /// * `format_settings` - Format settings to read from
    pub fn update_from_format(&mut self, format_settings: &FormatSettings) {
        self.font_family.clone_from(&format_settings.font_family);
        self.font_family_type = format_settings.font_family_type;
        self.font_style = format_settings.font_style;
        self.font_size = format_settings.font_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_recent_file() {
        let mut config = Config::create_default();
        // Use platform-agnostic test paths
        let path1 = if cfg!(windows) {
            r"C:\path\to\file1.txt"
        } else {
            "/path/to/file1.txt"
        };
        let path2 = if cfg!(windows) {
            r"C:\path\to\file2.txt"
        } else {
            "/path/to/file2.txt"
        };
        config.add_recent_file(path1);
        config.add_recent_file(path2);
        assert_eq!(config.recent_files.len(), 2);
        assert_eq!(config.recent_files[0], path2);
    }

    #[test]
    fn test_recent_files_limit() {
        let mut config = Config::create_default();
        for i in 0..15 {
            let path = if cfg!(windows) {
                format!(r"C:\path\to\file{i}.txt")
            } else {
                format!("/path/to/file{i}.txt")
            };
            config.add_recent_file(&path);
        }
        assert_eq!(config.recent_files.len(), 10);
    }
}
