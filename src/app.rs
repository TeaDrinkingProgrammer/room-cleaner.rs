use egui::{Color32, Key, Pos2, Rect, Stroke, Ui, Vec2};
use rand::Rng;
use serde::Serialize;

pub static SQUARE: f32 = 25.0;
pub static WIDTH_AND_HEIGHT: f32 = 50.0;

#[derive(Serialize, Debug, Clone)]
struct Object {
    rect: Rect,
    color: Color32,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            rect: Rect {
                min: Pos2::new(SQUARE * 1.0, SQUARE * 1.0),
                max: Pos2::new(SQUARE * 2.0, SQUARE * 2.0),
            },
            color: Color32::from_rgb(255, 190, 0),
        }
    }
}

impl Object {
    fn move_pos(&mut self, collision_rects: &Vec<Object>, x: f32, y: f32) {
        let translation = Vec2 { x, y };
        let moved_obj = self.rect.translate(translation);
        if collision_rects.iter().all(|obj: &Object| {
            let rect = obj.rect.expand2(Vec2 { x: -0.1, y: -0.1 });
            if rect.intersects(moved_obj) {
                info!("Collision detected, {:?}", obj.color);
                false
            } else {
                true
            }
        }) {
            self.rect = moved_obj;
        }
    }
}

#[derive(Default, Serialize)]
pub(crate) struct App {
    robot: Object,
    objects: Vec<Object>,
    todo: u16,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        let left_wall = Object {
            rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 {
                    x: SQUARE,
                    y: 24.0 * SQUARE,
                },
            },
            color: Color32::BLACK,
        };
        let right_wall = Object {
            rect: Rect {
                min: Pos2 {
                    x: 31.0 * SQUARE,
                    y: 0.0,
                },
                max: Pos2 {
                    x: 32.0 * SQUARE,
                    y: 24.0 * SQUARE,
                },
            },
            color: Color32::BLACK,
        };
        let upper_wall = Object {
            rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 {
                    x: 32.0 * SQUARE,
                    y: SQUARE,
                },
            },
            color: Color32::BLACK,
        };
        let lower_wall = Object {
            rect: Rect {
                min: Pos2 {
                    x: 0.0,
                    y: 23.0 * SQUARE,
                },
                max: Pos2 {
                    x: 32.0 * SQUARE,
                    y: 24.0 * SQUARE,
                },
            },
            color: Color32::BLACK,
        };
        let mut objects = generate_objects();
        objects.push(left_wall);
        objects.push(right_wall);
        objects.push(upper_wall);
        objects.push(lower_wall);
        let mut todo = 0;

        for x in 1..31 {
            for y in 1..23 {
                let rect = Rect {
                    min: Pos2 {
                        x: x as f32 * SQUARE,
                        y: y as f32 * SQUARE,
                    },
                    max: Pos2 {
                        x: (x + 1) as f32 * SQUARE,
                        y: (y + 1) as f32 * SQUARE,
                    },
                };
                if objects.iter().any(|obj| obj.rect.contains(rect.center())) {
                    info!("Object at {x}/{y}");
                } else {
                    todo += 1;
                }
            }
        }
        info!("TODO: {}", todo);

        Self {
            objects,
            todo,
            ..Self::default()
        }
    }

    fn keyboard_input(&mut self, ui: &Ui) {
        ui.input_mut(|i| {
            if i.key_pressed(Key::ArrowLeft) {
                info!("Moving keyboard Left");
                self.robot.move_pos(&self.objects, -SQUARE, 0.0);
            } else if i.key_pressed(Key::ArrowRight) {
                info!("Moving keyboard Right");
                self.robot.move_pos(&self.objects, SQUARE, 0.0);
            } else if i.key_pressed(Key::ArrowDown) {
                info!("Moving keyboard Down");
                self.robot.move_pos(&self.objects, 0.0, SQUARE);
            } else if i.key_pressed(Key::ArrowUp) {
                info!("Moving keyboard Up");
                self.robot.move_pos(&self.objects, 0.0, -SQUARE);
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
            for object in self.objects.iter() {
                let painter = ui.painter();
                painter.rect(object.rect, 0.0, object.color, Stroke::NONE);
            }
            self.keyboard_input(ui);
            ui.painter()
                .rect(self.robot.rect, 0.0, self.robot.color, Stroke::NONE);
            grid(ui, WIDTH_AND_HEIGHT * SQUARE, WIDTH_AND_HEIGHT * SQUARE);
            ui.colored_label(Color32::WHITE, format!("0/{}", self.todo))
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

fn generate_objects() -> Vec<Object> {
    let mut rng = rand::thread_rng();
    let mut num_objects = rng.gen_range(3..=6);
    let mut objects = Vec::new();
    let height = 24;
    let width = 31;
    for x in 3..width - 4 {
        for y in 3..height - 3 {
            let place = rng.gen_range(0..=100);
            let rect = Rect {
                min: Pos2::new(x as f32 * SQUARE, y as f32 * SQUARE),
                max: Pos2::new(
                    (x as f32 + rng.gen_range(4..=16) as f32) * SQUARE,
                    (y as f32 + rng.gen_range(4..=16) as f32) * SQUARE,
                ),
            };
            let rect_exp = rect.expand2(Vec2 {
                x: 2.0 * SQUARE,
                y: 2.0 * SQUARE,
            });
            if place >= 40
                && objects
                    .iter()
                    .all(|obj: &Object| !obj.rect.intersects(rect_exp))
            {
                objects.push(Object {
                    rect,
                    color: rand_color(),
                });
                num_objects -= 1;
                if num_objects == 0 {
                    return objects;
                }
            }
        }
    }
    objects
}

fn rand_color() -> Color32 {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..=6);
    let colors = vec![
        Color32::BLUE,
        Color32::RED,
        Color32::GREEN,
        Color32::YELLOW,
        Color32::LIGHT_RED,
        Color32::DARK_BLUE,
        Color32::KHAKI,
    ];
    colors[random]
}
