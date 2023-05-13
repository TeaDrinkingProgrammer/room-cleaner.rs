use egui::{Color32, Pos2, Rect, Vec2};
use rand::Rng;

use crate::{app::SQUARE, object::Object};

pub fn calculate_todo(objects: &[Object]) -> u16 {
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
    todo
}

pub fn generate_collision() -> Vec<Object> {
    let mut collision = Vec::new();
    collision.extend(generate_objects());
    collision.extend(generate_walls());
    collision
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

fn generate_walls() -> Vec<Object> {
    vec![
        Object {
            rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 {
                    x: SQUARE,
                    y: 24.0 * SQUARE,
                },
            },
            color: Color32::BLACK,
        },
        Object {
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
        },
        Object {
            rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 {
                    x: 32.0 * SQUARE,
                    y: SQUARE,
                },
            },
            color: Color32::BLACK,
        },
        Object {
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
        },
    ]
}

pub fn rand_color() -> Color32 {
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
