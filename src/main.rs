#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate log;
mod app;
use app::App;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    use egui::Vec2;

    use crate::app::{SQUARE, WIDTH_AND_HEIGHT};
    tracing_subscriber::fmt::init();
    info!("starting up");
    let window_size = Some( Vec2{
        x: SQUARE*WIDTH_AND_HEIGHT as f32,
        y: SQUARE*WIDTH_AND_HEIGHT as f32,
    });
    let native_options = eframe::NativeOptions{
        min_window_size: window_size,
        max_window_size: window_size,
        resizable: false,
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
