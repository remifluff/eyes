use crate::{Connection, Fbo, Model, PORT};
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

    pub fn draw_to_frame(&self, draw: &Draw) {
        draw.texture(&self.fbo.texture).w_h(200.0, 200.0);
        // .xy(self.position);
    }

    pub fn image_handler(mut buffer: Vec<u8>) {
        // print!("{:?}", buf);
        let mut send_data: Vec<u8> = Vec::new();

        for (pos, e) in buffer.iter().enumerate() {
            let col_index = pos % 12;
            let row_index = pos / 12;

            match pos {
                0 => send_data.push(255),
                _ if col_index == 0 => {
                    send_data.push(0);
                    send_data.push(clamp(*e, 0u8, 200u8));
                }
                _ if (row_index % 2) == 0 => {
                    send_data.push(clamp(*e, 0u8, 20u8));
                }
                _ => send_data.push(clamp(*e, 0u8, 200u8)),
            };
        }
        // ((pos / 12)) 0
        send_data.push(254);

        unsafe {
            PORT.write(send_data);
        }
    }

    pub fn send_to_screen(&self, a: &App) {
        self.fbo.snapshot_texture(a, Screen::image_handler)
    }
}
