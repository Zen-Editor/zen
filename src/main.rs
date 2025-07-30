#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ui;
pub mod helpers;
pub mod app;
pub mod config;

use eframe::egui;
use std::sync::Arc;
use app::ZenEditor;

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../assets/zen_logo.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();

    let size = width.min(height);
    let cropped_image = if width != height {
        let offset_x = (width - size) / 2;
        let offset_y = (height - size) / 2;
        image::imageops::crop_imm(&image, offset_x, offset_y, size, size).to_image()
    } else {
        image
    };

    let resized = image::imageops::resize(&cropped_image, 512, 512, image::imageops::FilterType::Lanczos3);

    egui::IconData {
        rgba: resized.into_raw(),
        width: 512,
        height: 512,
    }
}

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 600.0])
            .with_title("Zen Editor")
            .with_decorations(false)
            .with_transparent(true)
            .with_icon(Arc::new(load_icon()))
            .with_min_inner_size([800.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Zen Editor",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let editor = ZenEditor::default();
            Ok(Box::new(editor))
        }),
    )
}