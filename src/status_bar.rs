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
        ui.label(format!(
            "Ln {}, Col {}",
            editor_state.cursor_line, editor_state.cursor_column
        ));
    });
}

