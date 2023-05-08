use egui::{Color32, FontFamily, RichText, ScrollArea, Vec2};
use serde::Serialize;

#[derive(Default, Serialize)]
pub(crate) struct App {}

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
        // ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            // egui::menu::bar(ui, |ui| {
            //     if ui.button("Save").clicked() {
            //         self.service.request(Method::SaveAll);
            //     };
            // });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let tl = ui.ctx().available_rect().left_top().to_vec2();
            let br = ui.ctx().available_rect().right_bottom().to_vec2();
            let width = br.x - tl.x;
            let height = br.y - tl.y;

            let window = egui::Window::new("Canvas")
                .default_width(width)
                .default_height(height)
                .collapsible(false)
                .title_bar(false)
                .vscroll(false)
                .resizable(false)
                .fixed_pos(tl.to_pos2());
            window.show(ctx, |ui| {
                ctx.request_repaint();
            });
        });
    }
}
