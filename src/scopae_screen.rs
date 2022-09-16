use crate::Model;
use nannou::prelude::*;
use wgpu::TextueSnapshot;

mod eye;
use eye::Eye;

pub mod fbo;
use fbo::Fbo;

pub struct ScopaeScreen {
    pub fbo: Fbo,
    eye: Eye,
    pixel_resolution: Point2,
    screen_location: Rect,
    // write_timer: Timer,
    // vision_timer: Timer,
}

impl ScopaeScreen {
    pub fn new(a: &App, dim: Point2) -> ScopaeScreen {
        let frame_buffer = Fbo::new(a, dim);

        ScopaeScreen {
            fbo: frame_buffer,
            eye: Eye {
                x: (0.0),
                y: (0.0),
                r: (3.0),
                open_percent: (0.1),
            },
            pixel_resolution: todo!(),
            screen_location: todo!(),
        }
    }

    pub fn update(&mut self, eye_targer: Point2, t: f32) {
        self.eye.update(t);
    }

    fn render_fbo() {}

    pub fn draw_eye_to_fbo() {}

    pub fn draw_previews(&self, draw: &Draw) {
        draw.texture(&self.fbo.texture).w_h(20.0, 20.0);
        self.eye.draw(&draw);
    }

    fn draw(&self) -> &Draw {
        &self.fbo.draw();

        let draw = screen.draw();

        draw.background().color(WHITE);
        self.render(app);
        self.send_to_screen(app);
        self.draw_to_frame(app);
    }

    pub fn render(&self, a: &App) {
        self.fbo.render(a)
    }

    pub fn output_to_screens(&self, a: &App) {
        self.fbo.snapshot_texture(a, ScopaeScreen::image_handler)
    }
}
