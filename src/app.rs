//! Main application state and logic
//!
//! This module contains the `NodepatApp` struct which manages the overall
//! application state including document content, settings, and UI state.

use crate::config::Config;
use crate::editor::EditorState;
use crate::file_ops::FileState;
use crate::format::FormatSettings;
use crate::search::SearchState;
use eframe::egui;

/// Main application state
///
/// Manages all application state including document content,
/// file operations, editor settings, and UI state.
#[allow(clippy::struct_excessive_bools)]
pub struct NodepatApp {
    /// File-related state (path, modified flag, encoding)
    pub file_state: FileState,
    /// Editor state (text content, cursor position, undo/redo)
    pub editor_state: EditorState,
    /// Format settings (word wrap, font)
    pub format_settings: FormatSettings,
    /// Search state (find/replace text, options)
    pub search_state: SearchState,
    /// Status bar visibility
    pub show_status_bar: bool,
    /// Dialog states
    pub show_find_dialog: bool,
    pub show_replace_dialog: bool,
    pub show_font_dialog: bool,
    pub show_about_dialog: bool,
    pub show_goto_dialog: bool,
    pub show_open_dialog: bool,
    pub show_save_dialog: bool,
    pub show_page_setup_dialog: bool,
    pub goto_line: String,
    /// Configuration
    pub config: Config,
    /// Dark mode enabled
    pub dark_mode: bool,
}

impl Default for NodepatApp {
    fn default() -> Self {
        let config = Config::load();
        let mut app = Self {
            file_state: FileState::default(),
            editor_state: EditorState::default(),
            format_settings: FormatSettings::default(),
            search_state: SearchState::default(),
            show_status_bar: config.show_status_bar,
            show_find_dialog: false,
            show_replace_dialog: false,
            show_font_dialog: false,
            show_about_dialog: false,
            show_goto_dialog: false,
            show_open_dialog: false,
            show_save_dialog: false,
            show_page_setup_dialog: false,
            goto_line: String::new(),
            dark_mode: config.dark_mode,
            config,
        };
        // Apply config to format settings
        app.config.apply_to_format(&mut app.format_settings);
        app
    }
}

impl eframe::App for NodepatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update window title
        let title = if self.file_state.file_path.is_empty() {
            if self.file_state.is_modified {
                "Untitled* - Nodepat".to_string()
            } else {
                "Untitled - Nodepat".to_string()
            }
        } else {
            // Use PathBuf for cross-platform path handling
            let path = std::path::Path::new(&self.file_state.file_path);
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled");
            if self.file_state.is_modified {
                format!("{filename}* - Nodepat")
            } else {
                format!("{filename} - Nodepat")
            }
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));

        // Apply theme (light/dark mode)
        ctx.set_visuals(if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        // Apply font settings to the context before showing editor
        // Note: egui only supports built-in font families, so we always use Monospace
        // The font_family setting is kept for UI consistency but doesn't affect rendering
        let font_size = self.format_settings.font_size;
        ctx.style_mut(|style| {
            style.text_styles.insert(
                egui::TextStyle::Monospace,
                egui::FontId::monospace(font_size),
            );
        });

        // Show menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            crate::menu::show_menu_bar(ui, self);
        });

        // Show main text area - fill remaining space
        let editor_bg = if self.dark_mode {
            egui::Color32::from_rgb(30, 30, 30)
        } else {
            egui::Color32::from_rgb(255, 255, 255)
        };
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(editor_bg).inner_margin(0.0)) // Remove inner margin to maximize space
            .show(ctx, |ui| {
                // Handle Ctrl + Scroll for font size when over editor area
                // Check raw input events to detect scroll while Ctrl is held
                ui.input(|i| {
                    if i.modifiers.ctrl {
                        // Check for scroll events in raw input
                        for event in &i.events {
                            if let egui::Event::MouseWheel { delta, .. } = event {
                                let scroll_y = delta.y;
                                if scroll_y.abs() > 0.0 {
                                    // Increase or decrease font size based on scroll direction
                                    let old_size = self.format_settings.font_size;
                                    let new_size = if scroll_y > 0.0 {
                                        // Scroll up: increase font size
                                        (old_size + 1.0).min(72.0)
                                    } else {
                                        // Scroll down: decrease font size
                                        (old_size - 1.0).max(8.0)
                                    };

                                    if (new_size - old_size).abs() > 0.1 {
                                        self.format_settings.font_size = new_size;
                                        // Save to config
                                        self.config.update_from_format(&self.format_settings);
                                        let _ = self.config.save();
                                    }
                                }
                            }
                        }
                    }
                });
                crate::editor::show_editor(ui, self);
            });

        // Show status bar if enabled
        if self.show_status_bar {
            egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                crate::ui::status_bar::show_status_bar(ui, &self.editor_state);
            });
        }

        // Show dialogs
        crate::ui::dialogs::show_dialogs(ctx, self);

        // Save config on exit (would be better to do this in a proper cleanup)
        // For now, we'll save when settings change
    }
}
