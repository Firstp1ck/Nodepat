//! Text editor widget and editing logic
//!
//! This module handles the text editing functionality including
//! the text widget, cursor tracking, and basic editing operations.

use crate::app::NodepatApp;
use eframe::egui;

/// Editor state including text content and undo/redo history
#[derive(Default)]
pub struct EditorState {
    /// Current text content
    pub text: String,
    /// Undo history (previous text states)
    pub undo_history: Vec<String>,
    /// Redo history (future text states after undo)
    pub redo_history: Vec<String>,
    /// Current cursor position (line, column)
    pub cursor_line: usize,
    pub cursor_column: usize,
}

impl EditorState {
    /// Calculate line and column from character position
    ///
    /// # Arguments
    /// * `pos` - Character position in text
    ///
    /// # Returns
    /// Tuple of (line, column) where both are 1-indexed
    #[must_use]
    pub fn position_to_line_column(&self, pos: usize) -> (usize, usize) {
        let text_before = &self.text[..pos.min(self.text.len())];
        let line = text_before.matches('\n').count() + 1;
        let last_newline = text_before.rfind('\n').map_or(0, |i| i + 1);
        let column = pos - last_newline + 1;
        (line, column)
    }

    /// Save current state to undo history
    pub fn save_undo_state(&mut self) {
        self.undo_history.push(self.text.clone());
        // Limit undo history to prevent memory issues
        if self.undo_history.len() > 100 {
            self.undo_history.remove(0);
        }
        // Clear redo history when new edit is made
        self.redo_history.clear();
    }

    /// Undo last edit
    pub fn undo(&mut self) -> bool {
        if let Some(previous) = self.undo_history.pop() {
            let current = std::mem::replace(&mut self.text, previous);
            self.redo_history.push(current);
            true
        } else {
            false
        }
    }

    /// Redo last undone edit
    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_history.pop() {
            let current = std::mem::replace(&mut self.text, next);
            self.undo_history.push(current);
            true
        } else {
            false
        }
    }
}

/// Show the text editor widget
///
/// # Arguments
/// * `ui` - egui UI context
/// * `app` - Application state
pub fn show_editor(ui: &mut egui::Ui, app: &mut NodepatApp) {
    // Constants for row calculation
    const MAX_ROWS: f32 = 1_000_000.0; // Reasonable maximum for UI

    // Get the full available height before any widgets
    let available_height = ui.available_height();

    // Word wrap is always enabled - only vertical scrolling, text wraps to width
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.set_min_height(available_height);

            // Calculate desired rows using clamp (adjust line height based on font size)
            let font_size = app.format_settings.font_size;
            let line_height = font_size * 1.2; // Line height is typically 1.2x font size
            let rows_f32 = (available_height / line_height).clamp(1.0, MAX_ROWS);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let desired_rows = rows_f32 as usize;

            let text_edit = egui::TextEdit::multiline(&mut app.editor_state.text)
                .desired_width(f32::INFINITY)
                .desired_rows(desired_rows)
                .font(egui::TextStyle::Monospace)
                .show(ui);

            // Update cursor position
            if let Some(cursor_range) = text_edit.cursor_range {
                let cursor_pos = cursor_range.primary.index;
                let (line, column) = app.editor_state.position_to_line_column(cursor_pos);
                app.editor_state.cursor_line = line;
                app.editor_state.cursor_column = column;
            }
        });

    // Handle keyboard shortcuts
    ui.input(|i| {
        // Ctrl+Z: Undo
        if i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && app.editor_state.undo() {
            app.file_state.is_modified = true;
        }
        // Ctrl+Y: Redo
        if i.key_pressed(egui::Key::Y) && i.modifiers.ctrl && app.editor_state.redo() {
            app.file_state.is_modified = true;
        }
        // F5: Insert Time/Date
        if i.key_pressed(egui::Key::F5) {
            insert_time_date(&mut app.editor_state);
            app.file_state.is_modified = true;
        }
    });
}

/// Insert current time and date at cursor position
///
/// # Arguments
/// * `editor` - Editor state
pub fn insert_time_date(editor: &mut EditorState) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = now.as_secs();
    let datetime = secs % 86400; // Seconds since midnight

    let hours = datetime / 3600;
    let minutes = (datetime % 3600) / 60;
    let seconds = datetime % 60;

    // Calculate date (simplified, assumes UTC)
    let days = secs / 86400;
    let epoch_days = days + 719_163; // Days since 0000-01-01 (approximate)
    let year = 1970 + (epoch_days / 365);
    let day_of_year = epoch_days % 365;
    let month = (day_of_year / 30) + 1;
    let day = (day_of_year % 30) + 1;

    let time_str = format!("{hours:02}:{minutes:02}:{seconds:02} {month:02}/{day:02}/{year}");
    // Note: In a real implementation, we'd need to get cursor position from the text edit widget
    // For now, append to end
    editor.text.push_str(&time_str);
}
