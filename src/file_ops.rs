//! File operations (open, save, encoding)
//!
//! This module handles file operations including opening, saving,
//! encoding detection and conversion, and recent files management.

use encoding_rs::{Encoding, UTF_8};
use std::fs;

/// File state including path, modified flag, and encoding
#[derive(Default)]
pub struct FileState {
    /// Current file path
    pub file_path: String,
    /// Whether the file has been modified
    pub is_modified: bool,
    /// Current encoding
    pub encoding: String,
}

impl FileState {
    /// Load file from path
    ///
    /// # Arguments
    /// * `path` - File path to load
    ///
    /// # Returns
    /// Result containing the file content as String, or error message
    pub fn load_file(&mut self, path: &str) -> Result<String, String> {
        let file_data = fs::read(path).map_err(|e| format!("Failed to read file: {e}"))?;

        // Check file size
        if file_data.len() > 60_000 {
            return Err(
                "File is too large. Nodepat can only handle files up to ~58KB.".to_string(),
            );
        }

        // Detect encoding
        let (text, encoding_used, _) = if file_data.starts_with(&[0xFF, 0xFE]) {
            // UTF-16 LE BOM
            let utf16_data = &file_data[2..];
            let decoded = decode_utf16_le(utf16_data)?;
            (decoded.into(), "UTF-16 LE", true)
        } else if file_data.starts_with(&[0xFE, 0xFF]) {
            // UTF-16 BE BOM
            let utf16_data = &file_data[2..];
            let decoded = decode_utf16_be(utf16_data)?;
            (decoded.into(), "UTF-16 BE", true)
        } else if file_data.starts_with(&[0xEF, 0xBB, 0xBF]) {
            // UTF-8 BOM
            let (decoded, enc_used, _) = UTF_8.decode(&file_data[3..]);
            (decoded, enc_used.name(), true)
        } else {
            // Try UTF-8 first, fallback to ANSI/Latin1
            String::from_utf8(file_data.clone()).map_or_else(
                |_| {
                    // Fallback to Latin1 (ANSI)
                    let encoding = Encoding::for_label(b"latin1").unwrap_or(UTF_8);
                    let (decoded, enc_used, _) = encoding.decode(&file_data);
                    (decoded, enc_used.name(), false)
                },
                |text| (text.into(), "UTF-8", false),
            )
        };

        self.file_path = path.to_string();
        self.encoding = encoding_used.to_string();
        self.is_modified = false;

        Ok(text.to_string())
    }

    /// Add file to recent files in config
    ///
    /// # Arguments
    /// * `config` - Configuration to update
    pub fn add_to_recent_files(&self, config: &mut crate::config::Config) {
        if !self.file_path.is_empty() {
            config.add_recent_file(&self.file_path);
            let _ = config.save();
        }
    }

    /// Save file to path
    ///
    /// # Arguments
    /// * `path` - File path to save to
    /// * `content` - Content to save
    ///
    /// # Returns
    /// Result indicating success or error message
    pub fn save_file(&mut self, path: &str, content: &str) -> Result<(), String> {
        let bytes = match self.encoding.as_str() {
            "UTF-16 LE" => {
                let mut bytes = vec![0xFF, 0xFE]; // BOM
                bytes.extend(encode_utf16_le(content));
                bytes
            }
            "UTF-16 BE" => {
                let mut bytes = vec![0xFE, 0xFF]; // BOM
                bytes.extend(encode_utf16_be(content));
                bytes
            }
            "ANSI" | "Latin1" => {
                let encoding = Encoding::for_label(b"latin1").unwrap_or(UTF_8);
                encoding.encode(content).0.to_vec()
            }
            _ => content.as_bytes().to_vec(), // UTF-8 or unknown
        };

        fs::write(path, bytes).map_err(|e| format!("Failed to write file: {e}"))?;

        self.file_path = path.to_string();
        self.is_modified = false;

        Ok(())
    }
}

/// Decode UTF-16 LE bytes to string
///
/// # Arguments
/// * `bytes` - UTF-16 LE encoded bytes
///
/// # Returns
/// Decoded string or error
fn decode_utf16_le(bytes: &[u8]) -> Result<String, String> {
    if !bytes.len().is_multiple_of(2) {
        return Err("Invalid UTF-16 LE: odd number of bytes".to_string());
    }

    let u16_chars: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    String::from_utf16(&u16_chars).map_err(|e| format!("Invalid UTF-16 LE: {e}"))
}

/// Decode UTF-16 BE bytes to string
///
/// # Arguments
/// * `bytes` - UTF-16 BE encoded bytes
///
/// # Returns
/// Decoded string or error
fn decode_utf16_be(bytes: &[u8]) -> Result<String, String> {
    if !bytes.len().is_multiple_of(2) {
        return Err("Invalid UTF-16 BE: odd number of bytes".to_string());
    }

    let u16_chars: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect();

    String::from_utf16(&u16_chars).map_err(|e| format!("Invalid UTF-16 BE: {e}"))
}

/// Encode string to UTF-16 LE bytes
///
/// # Arguments
/// * `text` - Text to encode
///
/// # Returns
/// Encoded bytes
fn encode_utf16_le(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .flat_map(|c| c.to_le_bytes().to_vec())
        .collect()
}

/// Encode string to UTF-16 BE bytes
///
/// # Arguments
/// * `text` - Text to encode
///
/// # Returns
/// Encoded bytes
fn encode_utf16_be(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .flat_map(|c| c.to_be_bytes().to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_save_utf8() {
        let mut file_state = FileState::default();
        let test_content = "Hello, World!\nTest line 2";

        // Use std::env::temp_dir() for cross-platform temp directory
        let mut temp_path = std::env::temp_dir();
        temp_path.push("test_Nodepat_utf8.txt");
        let temp_path_str = temp_path
            .to_str()
            .expect("Failed to convert temp path to string");

        file_state
            .save_file(temp_path_str, test_content)
            .expect("Failed to save test file");

        // Load
        let loaded = file_state
            .load_file(temp_path_str)
            .expect("Failed to load test file");
        assert_eq!(loaded, test_content);

        // Cleanup
        let _ = fs::remove_file(&temp_path);
    }

    #[test]
    fn test_file_too_large() {
        let mut file_state = FileState::default();
        let large_content = "x".repeat(70_000);

        // Use std::env::temp_dir() for cross-platform temp directory
        let mut temp_path = std::env::temp_dir();
        temp_path.push("test_Nodepat_large.txt");
        let temp_path_str = temp_path
            .to_str()
            .expect("Failed to convert temp path to string");

        fs::write(&temp_path, large_content).expect("Failed to write large test file");

        let result = file_state.load_file(temp_path_str);
        assert!(result.is_err());
        let error_msg = result.expect_err("Expected error for large file");
        assert!(error_msg.contains("too large"));

        // Cleanup
        let _ = fs::remove_file(&temp_path);
    }
}
