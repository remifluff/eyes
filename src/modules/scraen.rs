use image::{imageops::FilterType, DynamicImage, Pixel};
use nannou::{ease, prelude::*};

// normal distribution between 0 and 1

pub mod fbo;
use fbo::Fbo;

use crate::settings::*;

use super::{Rotation, ScraenDim};

const UPSCALE_VAL: u32 = 3;

pub struct Scraen {
    pub fbo: Fbo,
    fbo_resolution: (u32, u32),
    window_transform: Affine2,
    rotation: Rotation,

    scraen_resolution: (u32, u32),
    scraen_texture: wgpu::Texture,

    blink: Blink,

    eye_open_percent: f32,
    eye_r: f32,
    eye_xy: Point2,

    fbo_rect: Rect,
    draw_rect: Rect,
    target_pos: Vec2,

    webcam_rect: Rect,
    target_acc: Vec2,
    target_vel: Vec2,
}

impl Scraen {
    pub fn new(app: &App, params: ScraenDim, webcam_rect: Rect) -> Scraen {
        let scraen_resolution = (params.rez, params.rez);
        let fbo_resolution = (params.rez * UPSCALE_VAL, params.rez * UPSCALE_VAL);

        let fbo_rect =
            Rect::from_x_y_w_h(0.0, 0.0, fbo_resolution.0 as f32, fbo_resolution.1 as f32);

        let draw_rect = Rect::from_x_y_w_h(
            params.xy.0,
            params.xy.1,
            params.wh.0 * SCRAEN_SCALE,
            params.wh.1 * -SCRAEN_SCALE,
        );
        // let fbo_resolution = params.(w * 20, h * 20);

        let frame_buffer = Fbo::new(app, (fbo_resolution.0, fbo_resolution.1));
        let img = &DynamicImage::new_rgb8(params.rez, params.rez);
        let texture = wgpu::Texture::from_image(app, img);
        let window_transform = Affine2::from_scale_angle_translation(
            draw_rect.wh() / fbo_rect.wh(),
            0.0,
            draw_rect.xy(),
        );

        Scraen {
            fbo: frame_buffer,
            fbo_resolution,

            window_transform,

            scraen_resolution,
            scraen_texture: texture,

            eye_open_percent: (0.1),

            fbo_rect,
            draw_rect,
            webcam_rect,

            eye_r: fbo_rect.h() / 4.0,
            eye_xy: Vec2::splat(0.0),
            target_pos: Vec2::splat(0.0),
            target_vel: Vec2::splat(0.0),
            target_acc: Vec2::splat(0.0),
            rotation: params.rotation,

            blink: Blink::new(
                BLINK_SECS_TO_CLOSE,
                BLINK_SECS_STAY_CLOSE,
                BLINK_SECS_TO_OPEN,
                BLINK_CHANCE_PER_FRAME,
            ),
        }
    }

    pub fn update(&mut self, app: &App, target: Point2, time: f64) {
        self.blink.update(time);

        //smooth target eye motion with accselatation
        let new_target_vel = self.target_pos - target;
        self.target_acc = self.target_vel - new_target_vel;
        self.target_pos = self.target_pos + self.target_acc * EYE_ACCELERATION;
        //turn target motion into rotation and distance from center
        let screen_center = self.draw_rect.xy();
        let dist = screen_center.distance(self.target_pos);
        let percent = dist / self.webcam_rect.wh().max_element() * 2.0;
        let max_length = self.fbo_rect.wh().min_element() / 2.0;
        //calculate xy from radius and theta
        let radius = max_length * percent;
        let theta = (self.target_pos - screen_center).normalize().angle();
        self.eye_xy = vec2(radius * theta.cos(), radius * theta.sin());
    }

    pub fn draw_eye(&self) {
        let draw = &self.fbo.draw();
        draw.background().color(BLACK);
        let rect_height = self.eye_r * self.blink.val;

        let rect_wh = vec2(self.eye_r * 2.0, rect_height);
        let rect_xy = vec2(0.0, self.eye_r - (rect_height / 2.0));

        draw.ellipse()
            .xy(self.eye_xy)
            .radius(self.eye_r)
            .color(WHITE);

        draw.rect()
            .xy(self.eye_xy - rect_xy)
            .wh(rect_wh)
            .color(BLACK);

        draw.rect()
            .xy(self.eye_xy + rect_xy)
            .wh(rect_wh)
            .color(BLACK);
    }

    pub fn render_texture(&mut self, app: &App) {
        self.fbo.render(app);
        self.fbo.snapshot_texture(app);

        if let Ok(image) = self.fbo.image_buffer.try_lock() {
            self.scraen_texture = wgpu::Texture::from_image(
                app,
                &image.resize_exact(
                    self.scraen_resolution.0,
                    self.scraen_resolution.1,
                    FilterType::Gaussian,
                ),
            );
        }
    }

    pub fn draw_to_frame(&self, draw: &Draw) {
        let t = self.window_transform;

        draw.texture(&self.scraen_texture)
            .xy(t.transform_point2(self.fbo_rect.xy()))
            .wh(t.transform_vector2(self.fbo_rect.wh()));

        draw.line()
            .start(self.draw_rect.xy())
            .end(self.window_transform.transform_point2(self.eye_xy))
            .color(GREY)
            .color(GREY);
    }

    pub fn serial_packet(&self) -> Option<Vec<u8>> {
        if let Ok(image) = self.fbo.image_buffer.try_lock() {
            let img = image.resize_exact(
                self.scraen_resolution.0,
                self.scraen_resolution.1,
                FilterType::Triangle,
            );

            let small_img = match self.rotation {
                Rotation::Rotate0 => img,
                Rotation::Rotate90 => img.rotate90(),
                Rotation::Rotate180 => img.rotate180(),
                Rotation::Rotate270 => img.rotate270(),
            };

            let mut itt = small_img
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
                })
                .collect::<Vec<u8>>();
            itt.pop();
            Some(itt)
        } else {
            None
        }
    }
}

type StartTime = f64;
enum State {
    Closing(StartTime),
    Closed(StartTime),
    Opening(StartTime),
    Dorment,
}
struct Blink {
    state: State,
    shutting_time: f64,
    closed_time: f64,
    opening_time: f64,
    chance: u32,
    val: f32,
}

impl Blink {
    fn new(shutting_time: f64, closed_time: f64, opening_time: f64, chance: f32) -> Blink {
        Blink {
            state: State::Dorment,
            shutting_time,
            closed_time,
            opening_time,
            chance: (1.0 / chance) as u32,
            val: 0.0,
        }
    }

    fn update(&mut self, time: f64) {
        self.val = match self.state {
            State::Closing(start_time) => {
                let t = time - start_time;
                if t < self.shutting_time {
                    ease::sine::ease_in(t, 0.0, 1.0, self.shutting_time) as f32
                } else {
                    self.state = State::Closed(time);
                    1.0
                }
            }
            State::Closed(start_time) => {
                let t = time - start_time;
                if t < self.closed_time {
                    1.0
                } else {
                    self.state = State::Opening(time);
                    1.0
                }
            }
            State::Opening(start_time) => {
                let t = time - start_time;
                if t < self.opening_time {
                    (1.0 - ease::sine::ease_out(t, 0.0, 1.0, self.opening_time) as f32)
                } else {
                    self.state = State::Dorment;
                    0.0
                }
            }
            State::Dorment => {
                if random_range(0, self.chance) < 1 {
                    self.state = State::Closing(time)
                };
                0.0
            }
        }
    }
}

struct EaseBlink {
    val: f32,
    time: f32,
    last_blink: f32,
    duration: f32,
    blink_in_progress: bool,
}

impl EaseBlink {
    fn new(duration: f32) -> EaseBlink {
        EaseBlink {
            val: 0.0,
            time: 0.0,
            last_blink: 0.0,
            duration,
            blink_in_progress: false,
        }
    }
    fn update(&mut self, time: f32) {
        let t = time % (self.duration * 2.0);
        self.val = ease::sine::ease_in_out(t, 0.0, 1.0, self.duration);
    }

    fn start_ease(&mut self) {
        self.blink_in_progress = true;
        // self.val = 0.0;
    }
}

// impl EaseExt for f32 {
//     fn blink_ease(&self, d: f32) -> f32 {
//         let t = *self % (d * 2.0);
//         ease::sine::ease_in_out(t, 0.0, 1.0, d)
//     }
// }
