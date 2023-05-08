use std::f32::consts::PI;

use egui::{Color32, FontFamily, Painter, Pos2, Rect, RichText, ScrollArea, Stroke, Vec2};
use rand::{Rng, thread_rng, distributions::uniform::SampleUniform, seq::SliceRandom, rngs::ThreadRng};
use serde::Serialize;
use tracing_subscriber::field::debug;

pub static SQUARE: f32 = 12.5;
pub static WIDTH_AND_HEIGHT: u8 = 25;


#[derive(Serialize, Debug, Clone)]
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

        let gen = || rand::thread_rng().gen_range(1..15);
        let random_square = move || SQUARE * gen() as f32;
        
        let object_generator = || {
            let mut set: Vec<Object> = vec![
                Object{ rect: Rect{min: Pos2{ x: 0.0, y: 0.0}, max: Pos2{x: SQUARE * 2.0, y: SQUARE * 2.0} }, color: Color32::GREEN}, //Plant
                Object{ rect: Rect{min: Pos2{ x: 0.0, y: 0.0}, max: Pos2{x: SQUARE * 4.0, y: SQUARE * 4.0} }, color: Color32::RED}, //Chair
                Object{ rect: Rect{min: Pos2{ x: 0.0, y: 0.0}, max: Pos2{x: SQUARE, y: SQUARE * 4.0} }, color: Color32::BLACK}, //TV
                Object{ rect: Rect{min: Pos2{ x: 0.0, y: 0.0}, max: Pos2{x: SQUARE * 4.0, y: SQUARE * 2.0} }, color: Color32::BLUE}, //Sofa
                ];
    
            let mut objects:Vec<Object> = Vec::new();
            let mut thread_rng = rand::thread_rng();
            let mut objects_range = thread_rng.gen_range(2..5);

            for y in 4..WIDTH_AND_HEIGHT-4 {
                for x in 4..WIDTH_AND_HEIGHT-4 {
                    if objects_range == 0 {
                        break;
                    }
                    if objects.iter().any(|object| object.rect.contains(Pos2{x: x as f32 * SQUARE,y: y as f32 * SQUARE})) {
                        continue;
                    }
                    if rand::random() {
                        todo!();
                    }
                }
            }
            objects
        };

        Self{
            objects: object_generator(),
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
