//! Find/Replace dialog and logic
//!
//! This module handles the Find and Replace functionality including
//! dialogs, search logic, and text replacement.

use crate::app::NodepatApp;

/// Search state including find/replace text and options
#[derive(Default)]
pub struct SearchState {
    /// Text to find
    pub find_text: String,
    /// Text to replace with
    pub replace_text: String,
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Search direction (true = down, false = up)
    pub search_down: bool,
    /// Current search position
    pub search_position: usize,
}

impl SearchState {}

/// Find next occurrence of search text
///
/// # Arguments
/// * `app` - Application state
///
/// # Returns
/// True if match found, false otherwise
pub fn find_next(app: &mut NodepatApp) -> bool {
    if app.search_state.find_text.is_empty() {
        return false;
    }

    let text = if app.search_state.case_sensitive {
        app.editor_state.text.clone()
    } else {
        app.editor_state.text.to_lowercase()
    };

    let search_text = if app.search_state.case_sensitive {
        app.search_state.find_text.clone()
    } else {
        app.search_state.find_text.to_lowercase()
    };

    let start_pos = if app.search_state.search_down {
        app.search_state.search_position
    } else {
        0
    };

    if app.search_state.search_down {
        if let Some(pos) = text[start_pos..].find(&search_text) {
            app.search_state.search_position = start_pos + pos + search_text.len();
            // TODO: Highlight/select the found text
            true
        } else {
            // Wrap around
            if let Some(pos) = text[..start_pos].find(&search_text) {
                app.search_state.search_position = pos + search_text.len();
                true
            } else {
                false
            }
        }
    } else {
        // Search up
        if let Some(pos) = text[..start_pos].rfind(&search_text) {
            app.search_state.search_position = pos;
            true
        } else {
            // Wrap around
            if let Some(pos) = text[start_pos..].rfind(&search_text) {
                app.search_state.search_position = start_pos + pos;
                true
            } else {
                false
            }
        }
    }
}

/// Replace current match
///
/// # Arguments
/// * `app` - Application state
///
/// # Returns
/// True if replacement was made, false otherwise
pub fn replace_current(app: &mut NodepatApp) -> bool {
    if app.search_state.find_text.is_empty() {
        return false;
    }

    let text = if app.search_state.case_sensitive {
        app.editor_state.text.clone()
    } else {
        app.editor_state.text.to_lowercase()
    };

    let search_text = if app.search_state.case_sensitive {
        app.search_state.find_text.clone()
    } else {
        app.search_state.find_text.to_lowercase()
    };

    if let Some(pos) = text.find(&search_text) {
        app.editor_state.save_undo_state();
        app.editor_state
            .text
            .replace_range(pos..pos + search_text.len(), &app.search_state.replace_text);
        app.file_state.is_modified = true;
        app.search_state.search_position = pos + app.search_state.replace_text.len();
        true
    } else {
        false
    }
}

/// Replace all occurrences
///
/// # Arguments
/// * `app` - Application state
///
/// # Returns
/// Number of replacements made
pub fn replace_all(app: &mut NodepatApp) -> usize {
    if app.search_state.find_text.is_empty() {
        return 0;
    }

    app.editor_state.save_undo_state();

    let mut count = 0;
    let search_text = &app.search_state.find_text;
    let replace_text = &app.search_state.replace_text;

    if app.search_state.case_sensitive {
        while app.editor_state.text.contains(search_text) {
            app.editor_state.text = app.editor_state.text.replacen(search_text, replace_text, 1);
            count += 1;
        }
    } else {
        // Case-insensitive replacement is more complex
        let mut text_lower = app.editor_state.text.to_lowercase();
        let search_lower = search_text.to_lowercase();

        while let Some(pos) = text_lower.find(&search_lower) {
            let end_pos = pos + search_text.len();
            app.editor_state
                .text
                .replace_range(pos..end_pos, replace_text);
            text_lower = app.editor_state.text.to_lowercase();
            count += 1;
        }
    }

    if count > 0 {
        app.file_state.is_modified = true;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::NodepatApp;

    #[test]
    fn test_find_next() {
        let mut app = NodepatApp::default();
        app.editor_state.text = "Hello World Hello".to_string();
        app.search_state.find_text = "Hello".to_string();
        app.search_state.case_sensitive = false;
        app.search_state.search_down = true;
        app.search_state.search_position = 0;

        assert!(find_next(&mut app));
        assert_eq!(app.search_state.search_position, 5);
    }

    #[test]
    fn test_replace_all() {
        let mut app = NodepatApp::default();
        app.editor_state.text = "Hello World Hello".to_string();
        app.search_state.find_text = "Hello".to_string();
        app.search_state.replace_text = "Hi".to_string();
        app.search_state.case_sensitive = true;

        let count = replace_all(&mut app);
        assert_eq!(count, 2);
        assert_eq!(app.editor_state.text, "Hi World Hi");
    }
}
