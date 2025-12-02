//! File dialogs, font dialog, and about dialog
//!
//! This module implements various dialogs including file open/save,
//! font selection, and about dialog.

use crate::app::NodepatApp;
use crate::format::FontFamily;
use crate::ui::file_browser::FileBrowser;
use eframe::egui;
use std::path::PathBuf;

/// Show all dialogs
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
pub fn show_dialogs(ctx: &egui::Context, app: &mut NodepatApp) {
    if app.show_find_dialog {
        show_find_dialog(ctx, app);
    }
    if app.show_replace_dialog {
        show_replace_dialog(ctx, app);
    }
    if app.show_font_dialog {
        show_font_dialog(ctx, app);
    }
    if app.show_about_dialog {
        show_about_dialog(ctx, app);
    }
    if app.show_goto_dialog {
        show_goto_dialog(ctx, app);
    }
    if app.show_open_dialog {
        show_open_dialog(ctx, app);
    }
    if app.show_save_dialog {
        show_save_dialog(ctx, app);
    }
}

/// Show Find dialog
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
fn show_find_dialog(ctx: &egui::Context, app: &mut NodepatApp) {
    egui::Window::new("Find")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Find what:");
                ui.text_edit_singleline(&mut app.search_state.find_text);

                ui.checkbox(&mut app.search_state.case_sensitive, "Match case");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut app.search_state.search_down, true, "Down");
                    ui.radio_value(&mut app.search_state.search_down, false, "Up");
                });

                ui.horizontal(|ui| {
                    if ui.button("Find Next").clicked() {
                        crate::search::find_next(app);
                    }
                    if ui.button("Cancel").clicked() {
                        app.show_find_dialog = false;
                    }
                });
            });
        });
}

/// Show Replace dialog
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
fn show_replace_dialog(ctx: &egui::Context, app: &mut NodepatApp) {
    egui::Window::new("Replace")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Find what:");
                ui.text_edit_singleline(&mut app.search_state.find_text);

                ui.label("Replace with:");
                ui.text_edit_singleline(&mut app.search_state.replace_text);

                ui.checkbox(&mut app.search_state.case_sensitive, "Match case");

                ui.horizontal(|ui| {
                    if ui.button("Find Next").clicked() {
                        crate::search::find_next(app);
                    }
                    if ui.button("Replace").clicked() {
                        crate::search::replace_current(app);
                    }
                    if ui.button("Replace All").clicked() {
                        let count = crate::search::replace_all(app);
                        // Could show a message about how many replacements were made
                        eprintln!("Replaced {count} occurrences");
                    }
                    if ui.button("Cancel").clicked() {
                        app.show_replace_dialog = false;
                    }
                });
            });
        });
}

/// Show Font dialog
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
fn show_font_dialog(ctx: &egui::Context, app: &mut NodepatApp) {
    egui::Window::new("Font")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Font family:");
                egui::ComboBox::from_id_salt("font_family")
                    .selected_text(app.format_settings.font_family_type.display_name())
                    .show_ui(ui, |ui| {
                        for family in FontFamily::all() {
                            ui.selectable_value(
                                &mut app.format_settings.font_family_type,
                                family,
                                family.display_name(),
                            );
                        }
                    });

                ui.label("Size:");
                ui.add(egui::Slider::new(
                    &mut app.format_settings.font_size,
                    8.0..=72.0,
                ));

                ui.separator();
                ui.label("Sample");
                // Show sample text with current font settings
                let font_id = match app.format_settings.font_family_type {
                    FontFamily::Monospace => egui::FontId::monospace(app.format_settings.font_size),
                    FontFamily::Proportional => {
                        egui::FontId::proportional(app.format_settings.font_size)
                    }
                };
                ui.style_mut()
                    .text_styles
                    .insert(egui::TextStyle::Body, font_id);
                ui.label("AaBbYyZz");

                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        app.config.update_from_format(&app.format_settings);
                        let _ = app.config.save();
                        app.show_font_dialog = false;
                    }
                    if ui.button("Cancel").clicked() {
                        app.show_font_dialog = false;
                    }
                });
            });
        });
}

/// Show About dialog
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
fn show_about_dialog(ctx: &egui::Context, app: &mut NodepatApp) {
    egui::Window::new("About")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Nodepat");
                ui.label("Version 0.1.0");
                ui.label("Cross-platform text editor");
                ui.separator();
                ui.label("A simple text editor built with Rust and egui.");
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        app.show_about_dialog = false;
                    }
                });
            });
        });
}

/// Show Go To dialog
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
fn show_goto_dialog(ctx: &egui::Context, app: &mut NodepatApp) {
    egui::Window::new("Go To Line")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Line number:");
                ui.text_edit_singleline(&mut app.goto_line);

                ui.horizontal(|ui| {
                    if ui.button("Go To").clicked() && app.goto_line.parse::<usize>().is_ok() {
                        // TODO: Implement go to line functionality
                        app.show_goto_dialog = false;
                    }
                    if ui.button("Cancel").clicked() {
                        app.show_goto_dialog = false;
                    }
                });
            });
        });
}

/// Show Open file dialog
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
fn show_open_dialog(ctx: &egui::Context, app: &mut NodepatApp) {
    // Initialize file browser if needed
    if app.file_browser.is_none() {
        let initial_path = if app.file_state.file_path.is_empty() {
            None
        } else {
            std::path::Path::new(&app.file_state.file_path).parent()
        };
        app.file_browser = Some(FileBrowser::new(
            initial_path,
            false,
            Some("txt".to_string()),
        ));
    }

    // Show file browser
    if let Some(ref mut browser) = app.file_browser
        && let Some(path) = browser.show(ctx, "Open File")
    {
        if path == PathBuf::from("") {
            // Cancelled
            app.file_browser = None;
            app.show_open_dialog = false;
            return;
        }

        if let Some(path_str) = path.to_str() {
            match app.file_state.load_file(path_str) {
                Ok(content) => {
                    app.editor_state.text = content;
                    app.editor_state.undo_history.clear();
                    app.editor_state.redo_history.clear();
                    app.file_state.add_to_recent_files(&mut app.config);
                }
                Err(e) => {
                    eprintln!("Error loading file: {e}");
                }
            }
        }
        app.file_browser = None;
        app.show_open_dialog = false;
    }
}

/// Show Save file dialog
///
/// # Arguments
/// * `ctx` - egui context
/// * `app` - Application state
fn show_save_dialog(ctx: &egui::Context, app: &mut NodepatApp) {
    // Initialize file browser if needed
    if app.file_browser.is_none() {
        let initial_path = if app.file_state.file_path.is_empty() {
            None
        } else {
            std::path::Path::new(&app.file_state.file_path).parent()
        };
        let mut browser = FileBrowser::new(initial_path, true, Some("txt".to_string()));
        // Set initial filename if available
        if !app.file_state.file_path.is_empty()
            && let Some(filename) = std::path::Path::new(&app.file_state.file_path)
                .file_name()
                .and_then(|n| n.to_str())
        {
            browser.set_selected_file(filename.to_string());
        }
        app.file_browser = Some(browser);
    }

    // Show file browser
    if let Some(ref mut browser) = app.file_browser
        && let Some(path) = browser.show(ctx, "Save File")
    {
        if path == PathBuf::from("") {
            // Cancelled
            app.file_browser = None;
            app.show_save_dialog = false;
            return;
        }

        if let Some(path_str) = path.to_str() {
            if let Err(e) = app.file_state.save_file(path_str, &app.editor_state.text) {
                eprintln!("Error saving file: {e}");
            } else {
                app.file_state.add_to_recent_files(&mut app.config);
            }
        }
        app.file_browser = None;
        app.show_save_dialog = false;
    }
}
