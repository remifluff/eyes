use crate::{serial_Output, Fbo, Model, PORT};
use nannou::prelude::*;
use wgpu::TextueSnapshot;

pub struct Screen {
    pub fbo: Fbo,

    pub dim: Point2,
    position: Point2,
}

impl Screen {
    pub fn new(a: &App, dim: Point2) -> Screen {
        let frame_buffer = Fbo::new(a, dim);
        Screen {
            fbo: frame_buffer,
            dim,
            position: Point2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self) -> &Draw {
        &self.fbo.draw()
    }

    pub fn render(&self, a: &App) {
        self.fbo.render(a)
    }

    pub fn draw_to_frame(&self, a: &App) {
        a.draw().texture(&self.fbo.texture).w_h(20.0, 20.0);
        // .xy(self.position);
    }

    pub fn send_to_screen(&self, a: &App) {
        self.fbo.snapshot_texture(a, Screen::image_handler)
    }
}
