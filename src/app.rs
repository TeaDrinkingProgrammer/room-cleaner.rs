use egui::{Color32, Key, Pos2, Rect, Stroke, Ui, Vec2};
use serde::Serialize;

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
                max: Pos2::new(SQUARE * 3.0, SQUARE * 3.0),
            },
            color: Color32::from_rgb(255, 190, 0),
        }
    }
}

impl Object {
    fn move_pos(&mut self, screen: Rect, x: f32, y: f32) {
        let translation = Vec2 { x, y };
        let moved_obj = self.rect.translate(translation);
        if in_bounds_check(screen, moved_obj) {
            self.rect = moved_obj;
        } else {
            info!("out of bounds");
        }
    }
}

fn in_bounds_check(screen: Rect, rect: Rect) -> bool {
    screen.contains_rect(rect)
    // ctx.screen_rect().contains_rect(rect)
    // ctx.available_rect().contains_rect(rect)
    // true
}

#[derive(Default, Serialize)]
pub(crate) struct App {
    robot: Object,
    objects: Vec<Object>,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn keyboard_input(&mut self, ui: &Ui) {
        let screen = ui.ctx().available_rect();
        ui.input_mut(|i| {
            if i.key_pressed(Key::ArrowLeft) {
                info!("Moving keyboard Left");
                self.robot.move_pos(screen, -SQUARE, 0.0);
            } else if i.key_pressed(Key::ArrowRight) {
                info!("Moving keyboard Right");
                self.robot.move_pos(screen, SQUARE, 0.0);
            } else if i.key_pressed(Key::ArrowDown) {
                info!("Moving keyboard Down");
                self.robot.move_pos(screen, 0.0, SQUARE);
            } else if i.key_pressed(Key::ArrowUp) {
                info!("Moving keyboard Up");
                self.robot.move_pos(screen, 0.0, -SQUARE);
            }
        });
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        egui::CentralPanel::default().show(ctx, |ui| {
            // for object in self.objects.iter() {
            //     print!("{:#?}", object);
            //     let painter = ui.painter();
            //     painter.rect(object.rect, 0.0, object.color, Stroke::NONE);
            // }
            self.keyboard_input(ui);
            ui.painter()
                .rect(self.robot.rect, 0.0, self.robot.color, Stroke::NONE);
            grid(ui, 800.0, 600.0);
        });
        // ctx.request_repaint();
    }
}

fn grid(ui: &Ui, width: f32, height: f32) {
    for i in (0..width as u32).step_by(SQUARE as usize) {
        let x = i as f32;
        ui.painter().line_segment(
            [Pos2::new(x, 0.0), Pos2::new(x, height)],
            Stroke::new(1.0, Color32::from_gray(200)),
        );
    }
    for i in (0..height as u32).step_by(SQUARE as usize) {
        let y = i as f32;
        ui.painter().line_segment(
            [Pos2::new(0.0, y), Pos2::new(width, y)],
            Stroke::new(1.0, Color32::from_gray(200)),
        );
    }
}
