#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate log;
mod app;
mod generators;
mod object;
use app::App;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use crate::app::{SQUARE, WIDTH_AND_HEIGHT};
    use egui::Vec2;
    tracing_subscriber::fmt::init();
    info!("starting up");
    let window_size = Some(Vec2 {
        x: SQUARE * WIDTH_AND_HEIGHT,
        y: SQUARE * WIDTH_AND_HEIGHT,
    });
    let native_options = eframe::NativeOptions {
        min_window_size: window_size,
        max_window_size: window_size,
        resizable: false,
        initial_window_size: window_size,
        ..eframe::NativeOptions::default()
    };
    match eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    ) {
        Ok(_) => info!("exiting cleanly"),
        Err(e) => error!("application exited with error: {}", e),
    }
}
