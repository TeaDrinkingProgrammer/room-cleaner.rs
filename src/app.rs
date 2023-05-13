use egui::{epaint::PathShape, Color32, Key, Pos2, Stroke, Ui};
use serde::Serialize;

use crate::{
    generators::{calculate_todo, generate_collision},
    object::Object,
};

pub static SQUARE: f32 = 25.0;
pub static WIDTH_AND_HEIGHT: f32 = 50.0;

#[derive(Serialize, Debug, Clone)]
enum Mode {
    Dfs,
    Manually,
}
impl Default for Mode {
    fn default() -> Self {
        match std::env::args().nth(1) {
            Some(s) => {
                if s.to_ascii_lowercase().starts_with("dfs") {
                    info!("DFS");
                    Mode::Dfs
                } else {
                    info!("Manually mode");
                    Mode::Manually
                }
            }
            None => Mode::Manually,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Visited {
    Visited,
    NotVisited,
    OutOfBounds,
}

#[derive(Default, Serialize)]
pub(crate) struct App {
    robot: Object,
    path: Vec<Pos2>,
    cleaned: Vec<Object>,
    objects: Vec<Object>,
    charging_point: Object,
    todo: u16,
    move_count: usize,
    mode: Mode,
}
impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        let mut objects = generate_collision();
        let todo = calculate_todo(&objects);
        info!("TODO: {}", todo);
        let robot = Object::new(&objects);
        // Push robot into collision rects to avoid charging point spawning on robot
        objects.push(robot.clone());
        let charging_point = Object::new(&objects);
        // Remove robot from collision rects
        objects.pop();
        Self {
            objects,
            todo,
            path: Vec::new(),
            cleaned: vec![Object {
                rect: charging_point.rect,
                color: Color32::GOLD,
            }],
            move_count: 0,
            robot,
            charging_point,
            ..Self::default()
        }
    }

    fn keyboard_input(&mut self, ui: &mut Ui) {
        ui.input_mut(|i| {
            if i.key_pressed(Key::ArrowLeft) {
                info!("Moving keyboard Left");
                self.move_robot(-SQUARE, 0.0);
            } else if i.key_pressed(Key::ArrowRight) {
                info!("Moving keyboard Right");
                self.move_robot(SQUARE, 0.0);
            } else if i.key_pressed(Key::ArrowDown) {
                info!("Moving keyboard Down");
                self.move_robot(0.0, SQUARE);
            } else if i.key_pressed(Key::ArrowUp) {
                info!("Moving keyboard Up");
                self.move_robot(0.0, -SQUARE);
            }
        });
    }
    fn move_robot(&mut self, x: f32, y: f32) -> Visited {
        let moved = self.robot.move_pos(&self.objects, x, y);
        match moved {
            Some(moved) => {
                self.move_count += 1;
                self.path.push(moved.center());
                if self.cleaned.iter().all(|obj| obj.rect != moved) {
                    info!("Cleaned {:?}", moved);
                    self.cleaned.push(Object {
                        rect: moved,
                        color: Color32::GOLD,
                    });
                    return Visited::Visited;
                }
                Visited::NotVisited
            }
            None => Visited::OutOfBounds,
        }
    }
    fn draw(&self, ui: &mut Ui) {
        let path = PathShape::line(self.path.clone(), Stroke::new(5.0, Color32::GREEN));
        ui.painter().add(path);
        // Charging point and robot
        ui.painter()
            .rect(self.robot.rect, 0.0, self.robot.color, Stroke::NONE);

        let charger_color = if self.robot.rect.contains(self.charging_point.rect.center()) {
            Color32::from_rgb(0, 190, 255)
        } else {
            Color32::from_rgb(0, 111, 190)
        };

        ui.painter().rect(
            self.charging_point.rect,
            0.0,
            charger_color,
            Stroke::new(8.0, Color32::BLACK),
        );
        grid(ui, WIDTH_AND_HEIGHT * SQUARE, WIDTH_AND_HEIGHT * SQUARE);
        // Score display
        ui.colored_label(
            Color32::WHITE,
            format!(
                "{}/{}  moved: {}",
                self.cleaned.len(),
                self.todo,
                self.move_count
            ),
        );
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default().show(ctx, |ui| {
            // Paint collision objects
            for object in self.objects.iter() {
                let painter = ui.painter();
                painter.rect(object.rect, 0.0, object.color, Stroke::NONE);
            }
            for object in self.cleaned.iter() {
                let painter = ui.painter();
                painter.rect(object.rect, 0.0, object.color, Stroke::NONE);
            }
            // User input
            match self.mode {
                Mode::Dfs => todo!(),
                Mode::Manually => self.keyboard_input(ui),
            }
            self.draw(ui);
        });
        ctx.request_repaint();
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
