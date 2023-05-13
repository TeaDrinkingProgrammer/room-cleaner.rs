use egui::{Color32, Rect, Pos2, Vec2};
use rand::Rng;
use serde::Serialize;

use crate::app::SQUARE;

#[derive(Serialize, Debug, Clone)]
pub struct Object {
    pub rect: Rect,
    pub color: Color32,
}

impl Default for Object {
    fn default() -> Self {
        let rect = Rect {
            min: Pos2::new(SQUARE * 1.0, SQUARE * 1.0),
            max: Pos2::new(SQUARE * 2.0, SQUARE * 2.0),
        };
        Self {
            rect,
            color: Color32::from_rgb(255, 190, 0),
        }
    }
}

impl Object {
    pub fn new(collision_rects: &[Object]) -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let rng_x = rng.gen_range(1..31);
            let rng_y = rng.gen_range(1..23);
            let rect = Rect {
                min: Pos2 {
                    x: rng_x as f32 * SQUARE,
                    y: rng_y as f32 * SQUARE,
                },
                max: Pos2 {
                    x: (rng_x + 1) as f32 * SQUARE,
                    y: (rng_y + 1) as f32 * SQUARE,
                },
            };
            if collision_rects.iter().all(|obj| !obj.rect.intersects(rect)) {
                return Self {
                    rect,
                    ..Self::default()
                };
            }
        }
    }

    pub fn move_pos(&mut self, collision_rects: &[Object], x: f32, y: f32) -> Option<Rect> {
        let translation = Vec2 { x, y };
        let moved_obj = self.rect.translate(translation);
        if collision_rects.iter().all(|obj: &Object| {
            let rect = obj.rect.shrink2(Vec2 { x: 0.1, y: 0.1 });
            if rect.intersects(moved_obj) {
                info!("Collision detected, {:?}", obj.color);
                false
            } else {
                true
            }
        }) {
            let pre_moved = self.rect;
            self.rect = moved_obj;
            Some(pre_moved)
        } else {
            None
        }
    }
}
