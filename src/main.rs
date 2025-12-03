//! Cross-platform text editor
//!
//! Cross-platform text editor built with Rust and egui.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod editor;
mod file_ops;
mod format;
mod menu;
mod search;
mod ui;

use app::NodepatApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Untitled - Nodepat")
            .with_inner_size([640.0, 480.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Nodepat",
        options,
        Box::new(|_cc| Ok(Box::<NodepatApp>::default())),
    )
}
