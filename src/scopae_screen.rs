use std::iter::Flatten;

use crate::{Connection, Model};
use image::{GenericImageView, Pixel};
use nannou::prelude::*;
use nannou::rand::seq::index;
use nannou::{ease, image::ImageBuffer, image::Pixels, prelude::*};
use wgpu::TextueSnapshot;

pub mod fbo;
use fbo::Fbo;

pub struct ScopaeScreen {
    pub fbo: Fbo,
    eye: Eye,
    resolution: (u32, u32),
    // pixel_resolution: Point2,
    // screen_location: Rect,
    // write_timer: Timer,
    // vision_timer: Timer,
}

impl ScopaeScreen {
    pub fn new(a: &App, resolution: (u32, u32)) -> ScopaeScreen {
        let frame_buffer = Fbo::new(a, resolution);
        ScopaeScreen {
            eye: Eye {
                x: (0.0),
                y: (0.0),
                r: (3.0),
                open_percent: (0.1),
            },
            fbo: frame_buffer,

            resolution,
        }
    }

    pub fn update(&mut self, app: &App, eye_target: Point2, time: f32) {
        // self.eye.set_center(eye_target);

        self.eye.update_openess(time);
    }
    pub fn render_texture(&self, app: &App) {
        let draw = &self.fbo.draw();

        draw.background().color(BLACK);
        self.eye.draw(draw);
        self.fbo.render(app);
        self.fbo.snapshot_texture(app);
    }
    fn draw(&self, draw: &Draw) {
        self.draw_to_frame(draw);
    }

    pub fn draw_to_frame(&self, draw: &Draw) {
        draw.texture(&self.fbo.texture).w_h(200.0, 200.0);
        // .xy(self.position);
    }
    pub fn serial_packet(&self) -> Option<Vec<u8>> {
        if let Ok(image) = self.fbo.image_buffer.try_lock() {
            // let mut port = connection;

            // image.clone().as_rgba8()?.rows().for_each(|row| {

            // ?.rows_mut().for_each(|row| {});

            let mut send_data: Vec<u8> = vec![255]
                .into_iter()
                .chain(
                    image
                        .clone()
                        .as_rgba8()?
                        .enumerate_rows()
                        .flat_map(|(i, row)| {
                            let mut mapped_row: Vec<u8> = row
                                .map(|(x, y, pix)| clamp(pix.to_luma().channels()[0], 0u8, 200u8))
                                .collect();
                            if i % 2 == 0 {
                                mapped_row.reverse();
                            }
                            mapped_row.push(0);
                            mapped_row
                        }),
                )
                .collect();

            send_data.push(254);

            Some(send_data)
        } else {
            None
        }
    }

    pub fn send_to_screens(&self, app: &App) {}
}
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

        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(self.r)
            .color(WHITE);

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
        // percent.blink_ease(5.0);
        // print!("{}", percent);
        self.open_percent = percent % 1.0;
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
        ease::sine::ease_in(t, 0.0, 1.0, d)
    }
}
// print!("{:?}", z);
// let mut send_data: Vec<u8> = Vec::new();

// for (pos, e) in buf.iter().enumerate() {
//     let col_index = pos as u32 % self.resolution.0;
//     let row_index = pos as u32 / self.resolution.1;

//     match pos {
//         0 => send_data.push(255),
//         _ if col_index == 0 => {
//             send_data.push(0);
//             send_data.push(clamp(*e, 0u8, 200u8));
//         }
//         _ if (row_index % 2) == 0 => {
//             send_data.push(clamp(*e, 0u8, 20u8));
//         }
//         _ => send_data.push(clamp(*e, 0u8, 200u8)),
//     };
// }
// ((pos / 12)) 0
