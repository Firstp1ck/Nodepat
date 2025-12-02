//! Status bar widget
//!
//! This module implements the status bar that displays
//! line and column position information.

use crate::editor::EditorState;
use eframe::egui;

/// Show the status bar
///
/// # Arguments
/// * `ui` - egui UI context
/// * `editor_state` - Editor state containing cursor position
pub fn show_status_bar(ui: &mut egui::Ui, editor_state: &EditorState) {
    ui.horizontal(|ui| {
        let line = editor_state.cursor_line;
        let col = editor_state.cursor_column;
        ui.label(format!("Ln {line}, Col {col}"));
    });
}
