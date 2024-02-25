#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    // implement logic to eframe
}

struct ScreenData {
    width: i32,
    height: i32,
    refresh_rate: i32,
    position: i32,
}
