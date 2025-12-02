//! File browser dialog
//!
//! This module provides a custom egui-based file browser dialog
//! for opening and saving files, replacing the rfd dependency.

use std::fs;
use std::path::{Path, PathBuf};

/// File browser dialog state
pub struct FileBrowser {
    /// Current directory path
    current_path: PathBuf,
    /// Selected file path (for save dialog)
    selected_file: String,
    /// File entries in current directory
    entries: Vec<FileEntry>,
    /// Error message to display
    error_message: String,
    /// Mode: true for save, false for open
    is_save_mode: bool,
    /// Filter for file extensions (e.g., "txt" for .txt files)
    file_filter: Option<String>,
}

/// File entry in directory listing
#[derive(Clone)]
struct FileEntry {
    /// Entry name
    name: String,
    /// Full path
    path: PathBuf,
    /// Is directory
    is_dir: bool,
}

impl FileBrowser {
    /// Create new file browser
    ///
    /// # Arguments
    /// * `initial_path` - Initial directory path (None for current directory)
    /// * `is_save_mode` - True for save dialog, false for open dialog
    /// * `file_filter` - Optional file extension filter (e.g., "txt")
    ///
    /// # Returns
    /// New `FileBrowser` instance
    #[must_use]
    pub fn new(
        initial_path: Option<&Path>,
        is_save_mode: bool,
        file_filter: Option<String>,
    ) -> Self {
        let current_path = initial_path
            .map(PathBuf::from)
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));

        let mut browser = Self {
            current_path,
            selected_file: String::new(),
            entries: Vec::new(),
            error_message: String::new(),
            is_save_mode,
            file_filter,
        };
        browser.refresh_entries();
        browser
    }

    /// Show file browser dialog
    ///
    /// # Arguments
    /// * `ctx` - egui context
    /// * `title` - Window title
    ///
    /// # Returns
    /// Some(path) if file selected, None if cancelled or still open
    #[allow(clippy::too_many_lines)]
    pub fn show(&mut self, ctx: &egui::Context, title: &str) -> Option<PathBuf> {
        let mut result = None;
        let mut should_close = false;

        egui::Window::new(title)
            .collapsible(false)
            .resizable(true)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Current path display and navigation
                    ui.horizontal(|ui| {
                        ui.label("Path:");
                        let mut path_str = self.current_path.to_string_lossy().to_string();
                        let path_edited = ui.text_edit_singleline(&mut path_str).changed();
                        if (path_edited && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            || ui.button("Go").clicked()
                        {
                            // Try to navigate to entered path
                            let new_path = PathBuf::from(&path_str);
                            if new_path.exists() && new_path.is_dir() {
                                self.current_path = new_path;
                                self.refresh_entries();
                            } else {
                                self.error_message = "Invalid directory path".to_string();
                            }
                        }
                    });

                    // Error message
                    if !self.error_message.is_empty() {
                        ui.colored_label(egui::Color32::RED, &self.error_message);
                    }

                    // File list
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            // Parent directory entry
                            if let Some(parent) = self.current_path.parent()
                                && ui.button(".. (Up)").clicked()
                            {
                                self.current_path = parent.to_path_buf();
                                self.refresh_entries();
                            }

                            // Directory and file entries
                            let mut clicked_dir: Option<PathBuf> = None;
                            let mut clicked_file: Option<String> = None;

                            for entry in &self.entries {
                                let label = if entry.is_dir {
                                    format!("📁 {}", entry.name)
                                } else {
                                    format!("📄 {}", entry.name)
                                };

                                if ui.button(&label).clicked() {
                                    if entry.is_dir {
                                        clicked_dir = Some(entry.path.clone());
                                    } else {
                                        clicked_file = Some(entry.name.clone());
                                    }
                                }
                            }

                            // Handle clicks after loop to avoid borrow conflicts
                            if let Some(dir_path) = clicked_dir {
                                self.current_path = dir_path;
                                self.refresh_entries();
                            }
                            if let Some(file_name) = clicked_file {
                                self.selected_file = file_name;
                            }
                        });

                    ui.separator();

                    // File name input (for save mode)
                    if self.is_save_mode {
                        ui.horizontal(|ui| {
                            ui.label("File name:");
                            ui.text_edit_singleline(&mut self.selected_file);
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Selected:");
                            ui.label(if self.selected_file.is_empty() {
                                "<none>"
                            } else {
                                &self.selected_file
                            });
                        });
                    }

                    // Buttons
                    ui.horizontal(|ui| {
                        let button_text = if self.is_save_mode { "Save" } else { "Open" };
                        let enabled = !self.selected_file.is_empty();

                        if ui
                            .add_enabled(enabled, egui::Button::new(button_text))
                            .clicked()
                        {
                            let file_path = self.current_path.join(&self.selected_file);

                            // Validate file path
                            if self.is_save_mode || file_path.exists() {
                                result = Some(file_path);
                                should_close = true;
                            } else {
                                self.error_message = "File does not exist".to_string();
                            }
                        }

                        if ui.button("Cancel").clicked() {
                            should_close = true;
                        }
                    });
                });
            });

        if should_close && result.is_none() {
            // Dialog was cancelled
            return Some(PathBuf::from("")); // Return empty path to indicate cancellation
        }

        result
    }

    /// Refresh directory entries
    fn refresh_entries(&mut self) {
        self.entries.clear();
        self.error_message.clear();

        match fs::read_dir(&self.current_path) {
            Ok(entries) => {
                let mut dirs = Vec::new();
                let mut files = Vec::new();

                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();

                    if path.is_dir() {
                        dirs.push(FileEntry {
                            name,
                            path,
                            is_dir: true,
                        });
                    } else if self.matches_filter(&name) {
                        files.push(FileEntry {
                            name,
                            path,
                            is_dir: false,
                        });
                    }
                }

                // Sort: directories first, then files, both alphabetically
                dirs.sort_by(|a, b| a.name.cmp(&b.name));
                files.sort_by(|a, b| a.name.cmp(&b.name));

                self.entries.extend(dirs);
                self.entries.extend(files);
            }
            Err(e) => {
                self.error_message = format!("Failed to read directory: {e}");
            }
        }
    }

    /// Set selected file name
    ///
    /// # Arguments
    /// * `filename` - File name to set
    pub fn set_selected_file(&mut self, filename: String) {
        self.selected_file = filename;
    }

    /// Check if file name matches filter
    ///
    /// # Arguments
    /// * `name` - File name to check
    ///
    /// # Returns
    /// True if matches filter or no filter set
    fn matches_filter(&self, name: &str) -> bool {
        self.file_filter
            .as_ref()
            .is_none_or(|filter| name.to_lowercase().ends_with(&format!(".{filter}")))
    }
}
