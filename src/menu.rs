//! Menu bar implementation
//!
//! This module implements the menu bar with File, Edit, Format,
//! View, and Help menus.

use crate::app::NodepatApp;
use eframe::egui;

/// Show the menu bar
///
/// # Arguments
/// * `ui` - egui UI context
/// * `app` - Application state
pub fn show_menu_bar(ui: &mut egui::Ui, app: &mut NodepatApp) {
    // Handle keyboard shortcuts
    ui.input(|i| {
        // Ctrl+N: New
        if i.key_pressed(egui::Key::N) && i.modifiers.ctrl {
            handle_new_file(app);
        }
        // Ctrl+O: Open
        if i.key_pressed(egui::Key::O) && i.modifiers.ctrl {
            app.show_open_dialog = true;
        }
        // Ctrl+S: Save
        if i.key_pressed(egui::Key::S) && i.modifiers.ctrl {
            handle_save(app);
        }
        // Ctrl+F: Find
        if i.key_pressed(egui::Key::F) && i.modifiers.ctrl {
            app.show_find_dialog = true;
        }
        // Ctrl+H: Replace
        if i.key_pressed(egui::Key::H) && i.modifiers.ctrl {
            app.show_replace_dialog = true;
        }
        // Ctrl+G: Go To
        if i.key_pressed(egui::Key::G) && i.modifiers.ctrl {
            app.show_goto_dialog = true;
        }
        // F3: Find Next
        if i.key_pressed(egui::Key::F3) {
            crate::search::find_next(app);
        }
    });
    egui::MenuBar::new().ui(ui, |ui| {
        show_file_menu(ui, app);
        show_edit_menu(ui, app);
        show_format_menu(ui, app);
        show_view_menu(ui, app);
        show_help_menu(ui, app);
    });
}

/// Show File menu
///
/// # Arguments
/// * `ui` - egui UI context
/// * `app` - Application state
fn show_file_menu(ui: &mut egui::Ui, app: &mut NodepatApp) {
    ui.menu_button("File", |ui| {
        if ui.button("New\tCtrl+N").clicked() {
            handle_new_file(app);
            ui.close();
        }
        if ui.button("Open...\tCtrl+O").clicked() {
            app.show_open_dialog = true;
            ui.close();
        }
        // Show recent files
        if !app.config.recent_files.is_empty() {
            ui.separator();
            for (idx, recent_file) in app.config.recent_files.iter().take(5).enumerate() {
                let label = if recent_file.len() > 50 {
                    format!("{}...", &recent_file[..50])
                } else {
                    recent_file.clone()
                };
                if ui.button(format!("{} {label}", idx + 1)).clicked() {
                    if let Ok(content) = app.file_state.load_file(recent_file) {
                        app.editor_state.text = content;
                        app.editor_state.undo_history.clear();
                        app.editor_state.redo_history.clear();
                    }
                    ui.close();
                }
            }
        }
        ui.separator();
        if ui.button("Save\tCtrl+S").clicked() {
            handle_save(app);
            ui.close();
        }
        if ui.button("Save As...").clicked() {
            app.show_save_dialog = true;
            ui.close();
        }
        ui.separator();
        if ui.button("Exit").clicked() {
            // Close the application
            // Note: In a full implementation, we would check for unsaved changes
            // and prompt the user to save before exiting
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            ui.close();
        }
    });
}

/// Show Edit menu
///
/// # Arguments
/// * `ui` - egui UI context
/// * `app` - Application state
fn show_edit_menu(ui: &mut egui::Ui, app: &mut NodepatApp) {
    ui.menu_button("Edit", |ui| {
        let can_undo = !app.editor_state.undo_history.is_empty();
        if ui
            .add_enabled(can_undo, egui::Button::new("Undo\tCtrl+Z"))
            .clicked()
        {
            if app.editor_state.undo() {
                app.file_state.is_modified = true;
            }
            ui.close();
        }
        let can_redo = !app.editor_state.redo_history.is_empty();
        if ui
            .add_enabled(can_redo, egui::Button::new("Redo\tCtrl+Y"))
            .clicked()
        {
            if app.editor_state.redo() {
                app.file_state.is_modified = true;
            }
            ui.close();
        }
        ui.separator();
        if ui.button("Cut\tCtrl+X").clicked() {
            handle_cut(app, ui.ctx());
            ui.close();
        }
        if ui.button("Copy\tCtrl+C").clicked() {
            handle_copy(app, ui.ctx());
            ui.close();
        }
        if ui.button("Paste\tCtrl+V").clicked() {
            handle_paste(app, ui.ctx());
            ui.close();
        }
        if ui.button("Delete\tDel").clicked() {
            handle_delete(app);
            ui.close();
        }
        ui.separator();
        if ui.button("Find...\tCtrl+F").clicked() {
            app.show_find_dialog = true;
            ui.close();
        }
        if ui.button("Find Next\tF3").clicked() {
            crate::search::find_next(app);
            ui.close();
        }
        if ui.button("Replace...\tCtrl+H").clicked() {
            app.show_replace_dialog = true;
            ui.close();
        }
        if ui.button("Go To...\tCtrl+G").clicked() {
            app.show_goto_dialog = true;
            ui.close();
        }
        ui.separator();
        if ui.button("Select All\tCtrl+A").clicked() {
            handle_select_all(app);
            // TextEdit handles Ctrl+A internally
            ui.close();
        }
        if ui.button("Time/Date\tF5").clicked() {
            crate::editor::insert_time_date(&mut app.editor_state);
            app.file_state.is_modified = true;
            ui.close();
        }
    });
}

/// Show Format menu
///
/// # Arguments
/// * `ui` - egui UI context
/// * `app` - Application state
fn show_format_menu(ui: &mut egui::Ui, app: &mut NodepatApp) {
    ui.menu_button("Format", |ui| {
        if ui.button("Font...").clicked() {
            app.show_font_dialog = true;
            ui.close();
        }
    });
}

/// Show View menu
///
/// # Arguments
/// * `ui` - egui UI context
/// * `app` - Application state
fn show_view_menu(ui: &mut egui::Ui, app: &mut NodepatApp) {
    ui.menu_button("View", |ui| {
        if ui.checkbox(&mut app.dark_mode, "Dark Mode").clicked() {
            app.config.dark_mode = app.dark_mode;
            let _ = app.config.save();
            ui.close();
        }
        ui.separator();
        if ui
            .checkbox(&mut app.show_status_bar, "Status Bar")
            .clicked()
        {
            app.config.show_status_bar = app.show_status_bar;
            let _ = app.config.save();
            ui.close();
        }
    });
}

/// Show Help menu
///
/// # Arguments
/// * `ui` - egui UI context
/// * `app` - Application state
fn show_help_menu(ui: &mut egui::Ui, app: &mut NodepatApp) {
    ui.menu_button("Help", |ui| {
        if ui.button("About").clicked() {
            app.show_about_dialog = true;
            ui.close();
        }
    });
}

/// Handle New File action
///
/// # Arguments
/// * `app` - Application state
fn handle_new_file(app: &mut NodepatApp) {
    // TODO: Check if file needs saving
    app.editor_state.text.clear();
    app.editor_state.undo_history.clear();
    app.editor_state.redo_history.clear();
    app.file_state.file_path.clear();
    app.file_state.is_modified = false;
}

/// Handle Save action
///
/// # Arguments
/// * `app` - Application state
fn handle_save(app: &mut NodepatApp) {
    if app.file_state.file_path.is_empty() {
        app.show_save_dialog = true;
    } else {
        let file_path = app.file_state.file_path.clone();
        let content = app.editor_state.text.clone();
        if let Err(e) = app.file_state.save_file(&file_path, &content) {
            // Show error dialog
            eprintln!("Save error: {e}");
        }
    }
}

/// Handle Cut action
///
/// # Arguments
/// * `app` - Application state
/// * `ctx` - egui context for clipboard access
fn handle_cut(app: &mut NodepatApp, _ctx: &egui::Context) {
    // TextEdit handles cut internally via Ctrl+X
    // We just mark as modified when cut happens
    app.editor_state.save_undo_state();
    app.file_state.is_modified = true;
}

/// Handle Copy action
///
/// # Arguments
/// * `_app` - Application state
/// * `_ctx` - egui context (`TextEdit` handles copy internally)
#[allow(clippy::missing_const_for_fn)] // Cannot be const: takes &mut
fn handle_copy(_app: &mut NodepatApp, _ctx: &egui::Context) {
    // TextEdit handles copy internally via Ctrl+C
}

/// Handle Paste action
///
/// # Arguments
/// * `app` - Application state
/// * `_ctx` - egui context (`TextEdit` handles paste internally)
fn handle_paste(app: &mut NodepatApp, _ctx: &egui::Context) {
    // TextEdit handles paste internally via Ctrl+V
    // We just mark as modified when paste happens
    app.editor_state.save_undo_state();
    app.file_state.is_modified = true;
}

/// Handle Delete action
///
/// # Arguments
/// * `app` - Application state
fn handle_delete(app: &mut NodepatApp) {
    // TextEdit handles delete internally
    app.editor_state.save_undo_state();
    app.file_state.is_modified = true;
}

/// Handle Select All action
///
/// # Arguments
/// * `_app` - Application state
#[allow(clippy::missing_const_for_fn)] // Cannot be const: takes &mut
fn handle_select_all(_app: &mut NodepatApp) {
    // TextEdit handles select all with Ctrl+A internally
    // This function is kept for menu consistency
}
