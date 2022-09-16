use crate::{Connection, Fbo, Model, PORT};
use nannou::{ease, prelude::*};
use wgpu::TextueSnapshot;

pub struct Eye {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub open_percent: f32,
}

impl Eye {
    pub fn draw(&self, draw: &Draw) {
        let rect_height = self.r * self.open_percent;
        let rect_offset = rect_height / 2.0;

        draw.ellipse().x_y(self.x, self.y).radius(self.r).color(RED);

        draw.rect()
            .x_y(self.x, self.y - self.r + rect_offset)
            .w(self.r * 2.0)
            .h(rect_height)
            .color(BLACK);

        draw.rect()
            .x_y(self.x, self.y + self.r - rect_offset)
            .w(self.r * 2.0)
            .h(rect_height)
            .color(BLACK);
    }

    pub fn set_center(&mut self, point: Point2) {
        self.x = point.x;
        self.y = point.y;
    }

    pub fn update_openess(&mut self, percent: f32) {
        percent.blink_ease(1.0);
        self.open_percent = percent;
    }
}
trait EaseExt {
    fn blink_ease(&self, d: f32) -> f32 {
        0.0
    }
}
impl EaseExt for f32 {
    fn blink_ease(&self, d: f32) -> f32 {
        let t = *self % (d * 2.0);
        ease::sine::ease_in_out(t, 0.0, 1.0, d)
    }
}
