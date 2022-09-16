<<<<<<< HEAD:src/screen.rs
use crate::{serial_Output, Fbo, Model, PORT};
=======
use crate::{Connection, Model, PORT};
>>>>>>> 7c65c0e987f48d6d5dabee4ea630ec978218182d:src/scopae_screen.rs
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

    pub fn update(&self, eye_targer: Point2) {
        // .eye.set_center(app.mouse.position());

        self.eye.update_openess(t);
    }
    fn draw(&self) -> &Draw {
        &self.fbo.draw()
        let draw = screen.draw();

        draw.background().color(WHITE);
        screen.render(app);
        screen.send_to_screen(app);
        screen.draw_to_frame(app);
    }

    pub fn render(&self, a: &App) {
        self.fbo.render(a)
    }

    pub fn draw_to_frame(&self, draw: &Draw) {
        draw.texture(&self.fbo.texture).w_h(20.0, 20.0);
        self.eye.draw(&draw);

        // .xy(self.position);
    }

    pub fn send_to_screen(&self, a: &App) {
        self.fbo.snapshot_texture(a, ScopaeScreen::image_handler)
    }
}
