//! Configuration file management
//!
//! This module handles loading and saving configuration from config.jsonc
//! including recent files, font settings, and window preferences.

use crate::format::{FontFamily, FontStyle, FormatSettings};
use std::fs;
use std::path::PathBuf;

/// Configuration structure
#[derive(Debug)]
pub struct Config {
    /// Recent files list
    pub recent_files: Vec<String>,
    /// Font family (kept for backward compatibility)
    pub font_family: String,
    /// Font family type (Monospace or Proportional)
    pub font_family_type: FontFamily,
    /// Font style (Regular, Bold, Italic, `BoldItalic`)
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
        if let Ok(content) = fs::read_to_string(&config_path)
            && let Ok(config) = Self::parse_json(&content)
        {
            return config;
        }
        Self::create_default()
    }

    /// Parse JSON string into Config
    ///
    /// # Arguments
    /// * `json` - JSON string to parse
    ///
    /// # Returns
    /// Config struct or error
    fn parse_json(json: &str) -> Result<Self, String> {
        let mut config = Self::create_default();
        let json = json.trim();

        // Remove outer braces
        let json = json
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or_else(|| "Invalid JSON: missing braces".to_string())?;

        // Parse each field
        for part in Self::split_json_fields(json) {
            let (key, value) = Self::parse_field(part)?;
            match key {
                "recent_files" => {
                    config.recent_files = Self::parse_string_array(value)?;
                }
                "font_family" => {
                    config.font_family = Self::parse_string(value)?;
                }
                "font_family_type" => {
                    config.font_family_type = Self::parse_font_family(value)?;
                }
                "font_style" => {
                    config.font_style = Self::parse_font_style(value)?;
                }
                "font_size" => {
                    if let Ok(size) = value.trim().parse::<f32>() {
                        config.font_size = size;
                    }
                }
                "show_status_bar" => {
                    config.show_status_bar = Self::parse_bool(value)?;
                }
                "dark_mode" => {
                    config.dark_mode = Self::parse_bool(value)?;
                }
                "window_width" => {
                    if let Ok(width) = value.trim().parse::<f32>() {
                        config.window_width = width;
                    }
                }
                "window_height" => {
                    if let Ok(height) = value.trim().parse::<f32>() {
                        config.window_height = height;
                    }
                }
                _ => {
                    // Ignore unknown fields
                }
            }
        }

        Ok(config)
    }

    /// Split JSON fields, handling nested structures
    ///
    /// # Arguments
    /// * `json` - JSON content without outer braces
    ///
    /// # Returns
    /// Vector of field strings
    fn split_json_fields(json: &str) -> Vec<&str> {
        let mut fields = Vec::new();
        let mut start = 0;
        let mut depth = 0;
        let mut in_string = false;
        let mut escape_next = false;

        for (i, ch) in json.char_indices() {
            if escape_next {
                escape_next = false;
                continue;
            }
            if ch == '\\' {
                escape_next = true;
                continue;
            }
            if ch == '"' {
                in_string = !in_string;
                continue;
            }
            if in_string {
                continue;
            }
            match ch {
                '{' | '[' => depth += 1,
                '}' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    fields.push(&json[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
        if start < json.len() {
            fields.push(&json[start..]);
        }
        fields
    }

    /// Parse a JSON field into key-value pair
    ///
    /// # Arguments
    /// * `field` - Field string (e.g., "key": value)
    ///
    /// # Returns
    /// Tuple of (key, value) or error
    fn parse_field(field: &str) -> Result<(&str, &str), String> {
        let field = field.trim();
        let colon_pos = field
            .find(':')
            .ok_or_else(|| "Invalid JSON field: missing colon".to_string())?;
        let key = field[..colon_pos].trim();
        let value = field[colon_pos + 1..].trim();

        // Remove quotes from key
        let key = key
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .ok_or_else(|| "Invalid JSON key: missing quotes".to_string())?;

        Ok((key, value))
    }

    /// Parse JSON string value
    ///
    /// # Arguments
    /// * `value` - JSON string value
    ///
    /// # Returns
    /// Parsed string or error
    fn parse_string(value: &str) -> Result<String, String> {
        let value = value.trim();
        value
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .map(|s| {
                s.replace("\\\"", "\"")
                    .replace("\\\\", "\\")
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\t", "\t")
            })
            .ok_or_else(|| "Invalid JSON string".to_string())
    }

    /// Parse JSON boolean value
    ///
    /// # Arguments
    /// * `value` - JSON boolean value
    ///
    /// # Returns
    /// Parsed boolean or error
    fn parse_bool(value: &str) -> Result<bool, String> {
        match value.trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err("Invalid boolean value".to_string()),
        }
    }

    /// Parse JSON array of strings
    ///
    /// # Arguments
    /// * `value` - JSON array value
    ///
    /// # Returns
    /// Vector of strings or error
    fn parse_string_array(value: &str) -> Result<Vec<String>, String> {
        let value = value.trim();
        let array_content = value
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or_else(|| "Invalid JSON array: missing brackets".to_string())?;

        if array_content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut strings = Vec::new();
        for item in Self::split_json_array_items(array_content) {
            strings.push(Self::parse_string(item.trim())?);
        }
        Ok(strings)
    }

    /// Split JSON array items
    ///
    /// # Arguments
    /// * `array_content` - Array content without brackets
    ///
    /// # Returns
    /// Vector of item strings
    fn split_json_array_items(array_content: &str) -> Vec<&str> {
        let mut items = Vec::new();
        let mut start = 0;
        let mut in_string = false;
        let mut escape_next = false;

        for (i, ch) in array_content.char_indices() {
            if escape_next {
                escape_next = false;
                continue;
            }
            if ch == '\\' {
                escape_next = true;
                continue;
            }
            if ch == '"' {
                in_string = !in_string;
                continue;
            }
            if in_string {
                continue;
            }
            if ch == ',' {
                items.push(&array_content[start..i]);
                start = i + 1;
            }
        }
        if start < array_content.len() {
            items.push(&array_content[start..]);
        }
        items
    }

    /// Parse `FontFamily` enum from JSON
    ///
    /// # Arguments
    /// * `value` - JSON string value
    ///
    /// # Returns
    /// `FontFamily` or error
    fn parse_font_family(value: &str) -> Result<FontFamily, String> {
        let value = Self::parse_string(value)?;
        match value.to_lowercase().as_str() {
            "monospace" => Ok(FontFamily::Monospace),
            "proportional" => Ok(FontFamily::Proportional),
            _ => Ok(FontFamily::default()),
        }
    }

    /// Parse `FontStyle` enum from JSON
    ///
    /// # Arguments
    /// * `value` - JSON string value
    ///
    /// # Returns
    /// `FontStyle` or error
    fn parse_font_style(value: &str) -> Result<FontStyle, String> {
        let value = Self::parse_string(value)?;
        match value.to_lowercase().as_str() {
            "regular" => Ok(FontStyle::Regular),
            "bold" => Ok(FontStyle::Bold),
            "italic" => Ok(FontStyle::Italic),
            "bolditalic" | "bold_italic" => Ok(FontStyle::BoldItalic),
            _ => Ok(FontStyle::default()),
        }
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

        let json = self.to_json();
        fs::write(&config_path, json).map_err(|e| format!("Failed to write config: {e}"))?;
        Ok(())
    }

    /// Convert Config to JSON string
    ///
    /// # Returns
    /// JSON string representation
    fn to_json(&self) -> String {
        use std::fmt::Write;
        let mut json = String::from("{\n");
        let _ = writeln!(
            json,
            "  \"recent_files\": {},",
            Self::string_array_to_json(&self.recent_files)
        );
        let _ = writeln!(
            json,
            "  \"font_family\": {},",
            Self::string_to_json(&self.font_family)
        );
        let _ = writeln!(
            json,
            "  \"font_family_type\": {},",
            Self::font_family_to_json(self.font_family_type)
        );
        let _ = writeln!(
            json,
            "  \"font_style\": {},",
            Self::font_style_to_json(self.font_style)
        );
        let _ = writeln!(json, "  \"font_size\": {},", self.font_size);
        let _ = writeln!(json, "  \"show_status_bar\": {},", self.show_status_bar);
        let _ = writeln!(json, "  \"dark_mode\": {},", self.dark_mode);
        let _ = writeln!(json, "  \"window_width\": {},", self.window_width);
        let _ = writeln!(json, "  \"window_height\": {}", self.window_height);
        json.push('}');
        json
    }

    /// Convert string to JSON string value
    ///
    /// # Arguments
    /// * `s` - String to convert
    ///
    /// # Returns
    /// JSON string representation
    fn string_to_json(s: &str) -> String {
        format!(
            "\"{}\"",
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
        )
    }

    /// Convert string array to JSON array
    ///
    /// # Arguments
    /// * `arr` - Array of strings
    ///
    /// # Returns
    /// JSON array representation
    fn string_array_to_json(arr: &[String]) -> String {
        if arr.is_empty() {
            return "[]".to_string();
        }
        let items: Vec<String> = arr.iter().map(|s| Self::string_to_json(s)).collect();
        format!("[{}]", items.join(", "))
    }

    /// Convert `FontFamily` to JSON string
    ///
    /// # Arguments
    /// * `family` - `FontFamily` enum value
    ///
    /// # Returns
    /// JSON string representation
    fn font_family_to_json(family: FontFamily) -> String {
        let name = match family {
            FontFamily::Monospace => "monospace",
            FontFamily::Proportional => "proportional",
        };
        Self::string_to_json(name)
    }

    /// Convert `FontStyle` to JSON string
    ///
    /// # Arguments
    /// * `style` - `FontStyle` enum value
    ///
    /// # Returns
    /// JSON string representation
    fn font_style_to_json(style: FontStyle) -> String {
        let name = match style {
            FontStyle::Regular => "regular",
            FontStyle::Bold => "bold",
            FontStyle::Italic => "italic",
            FontStyle::BoldItalic => "bolditalic",
        };
        Self::string_to_json(name)
    }

    /// Get configuration file path
    ///
    /// # Returns
    /// Path to config.jsonc file
    #[must_use]
    fn config_path() -> PathBuf {
        let mut path = if cfg!(windows) {
            std::env::var("APPDATA").map_or_else(|_| PathBuf::from("."), PathBuf::from)
        } else {
            std::env::var("HOME").map_or_else(
                |_| PathBuf::from("."),
                |home| PathBuf::from(home).join(".config"),
            )
        };
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
