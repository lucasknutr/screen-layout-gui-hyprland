#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    // implement logic to eframe
    eframe::run_native(
        "Screen Layout GUI for Hyprland",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default();
        })
    )
}

struct ScreenData {
    width: i32,
    height: i32,
    refresh_rate: i32,
    position: i32,
}

impl Default for ScreenData {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            refresh_rate: 144,
            position: 0,
        }
    }
}
