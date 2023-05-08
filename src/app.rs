use std::f32::consts::PI;

use egui::{Color32, FontFamily, Painter, Pos2, Rect, RichText, ScrollArea, Stroke, Vec2};
use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};
use serde::Serialize;
use tracing_subscriber::field::debug;

pub static SQUARE: f32 = 12.5;


#[derive(Serialize, Debug)]
struct Object {
    rect: Rect,
    color: Color32,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            rect: Rect {
                min: Pos2::new(0.0, 0.0),
                max: Pos2::new(SQUARE, SQUARE),
            },
            color: Color32::GOLD,
        }
    }
}

impl Object {
    fn move_pos(&mut self, ctx: &egui::Context, painter: &Painter, x: f32, y: f32) {
        let translation = Vec2 { x, y };
        self.rect = self.rect.translate(translation);
        if !in_bounds_check(ctx, self.rect) {
            println!("out of bounds");
        }
        painter.rect(self.rect, 0.0, Color32::GOLD, Stroke::NONE);
    }
}

fn in_bounds_check(ctx: &egui::Context, rect: Rect) -> bool{
    ctx.available_rect().contains_rect(rect)
}

#[derive(Default, Serialize)]
pub(crate) struct App {
    robot: Object,
    objects: Vec<Object>
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let rec = Object{ rect: Rect{min: Pos2{ x: SQUARE * 7.0, y: SQUARE * 7.0}, max: Pos2{x: SQUARE * 10.0, y: SQUARE * 10.0} },
            color: Color32::RED};
        let objects = vec![vec![rec]];

        let gen = move || rand::thread_rng().gen_range(1..15);
        let random_square = move || SQUARE * gen() as f32;
        
        let object_generator = move || {
            let x0 = random_square();
            let y0 = random_square();
            Object{ rect: Rect{min: Pos2{ x: x0, y: y0}, max: Pos2{x: x0 + random_square(), y: y0 + random_square()} }, color: Color32::RED}
        };

        let non_overlapping_generator = || {
            let mut objects: Vec<Object> = vec![];
            let len = objects.len();
            while len < 10 {
                let object = object_generator();
                for x in objects {
                    if x.rect.contains_rect(object.rect){
                        break;
                    }
                }
            }
        };

        Self{
            objects: (0..10).into_iter().map(|_| object_generator()).collect(),
            ..Self::default()
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            for object in self.objects.iter() {
                print!("{:#?}", object);
                painter.rect(object.rect, 0.0, object.color, Stroke::NONE);
            }
            ctx.request_repaint();
            // painter.rect(Rect{ min: Pos2{ x: SQUARE * 7.0, y: SQUARE * 7.0}, max: Pos2{x: SQUARE * 10.0, y: SQUARE * 10.0} }, 0.0, Color32::RED, Stroke::NONE);
            self.robot.move_pos(ctx, painter, SQUARE, 0.0);
            std::thread::sleep(std::time::Duration::from_millis(600));
        });
    }
}
