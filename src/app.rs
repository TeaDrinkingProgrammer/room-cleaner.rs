use std::f32::consts::PI;

use egui::{Color32, FontFamily, Painter, Pos2, Rect, RichText, ScrollArea, Stroke, Vec2};
use serde::Serialize;

pub static PIXEL: f32 = 25.0;

#[derive(Serialize)]
struct Robot {
    rect: Rect,
    color: Color32,
}

impl Default for Robot {
    fn default() -> Self {
        Self {
            rect: Rect {
                min: Pos2::new(0.0, 0.0),
                max: Pos2::new(PIXEL, PIXEL),
            },
            color: Color32::GOLD,
        }
    }
}

impl Robot {
    fn move_pos(&mut self, painter: &Painter, x: f32, y: f32) {
        let translation = Vec2 { x, y };
        self.rect = self.rect.translate(translation);
        painter.rect(self.rect, 0.0, Color32::GOLD, Stroke::NONE);
    }
}

#[derive(Default, Serialize)]
pub(crate) struct App {
    robot: Robot,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();
            let painter = ui.painter();
            self.robot.move_pos(painter, PIXEL, 0.0);
            std::thread::sleep(std::time::Duration::from_millis(600));
        });
    }
}
