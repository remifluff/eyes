use std::iter::Flatten;

use crate::{Connection, Model, ScraenDim, SCRAEN_SCALE};
use image::{imageops::FilterType, math, DynamicImage, GenericImageView, Pixel};
use nannou::{
    ease,
    geom::rect,
    image::{ImageBuffer, Pixels},
    lyon::math::Point,
    prelude::*,
    prelude::*,
    rand::seq::index,
};
use wgpu::TextueSnapshot;

pub mod fbo;
use fbo::Fbo;

const UPSCALE_VAL: u32 = 20;

pub struct Scraen {
    pub fbo: Fbo,
    fbo_resolution: (u32, u32),
    window_transform: Affine2,

    scraen_resolution: (u32, u32),
    scraen_texture: wgpu::Texture,

    eye_open_percent: f32,
    eye_r: f32,
    screen_center: Point2,
    eye_xy: Point2,
    eye_rt: Point2,

    fbo_rect: Rect,
    draw_rect: Rect,
    target: Vec2,

    blink_ease: EaseBlink,
}

impl Scraen {
    pub fn new(app: &App, screen_dim: ScraenDim) -> Scraen {
        let scraen_resolution = (screen_dim.rez, screen_dim.rez);
        let fbo_resolution = (screen_dim.rez * UPSCALE_VAL, screen_dim.rez * UPSCALE_VAL);

        let fbo_rect =
            Rect::from_x_y_w_h(0.0, 0.0, fbo_resolution.0 as f32, fbo_resolution.1 as f32);

        let draw_rect = Rect::from_x_y_w_h(
            screen_dim.xy.0,
            screen_dim.xy.1,
            screen_dim.wh.0 * SCRAEN_SCALE,
            screen_dim.wh.1 * -SCRAEN_SCALE,
        );
        // let fbo_resolution = screen_dim.(w * 20, h * 20);

        let frame_buffer = Fbo::new(app, (fbo_resolution.0, fbo_resolution.1));

        let texture =
            wgpu::Texture::from_image(app, &DynamicImage::new_rgb8(screen_dim.rez, screen_dim.rez));

        // let fbo_rect = Rect::from_x_y_w_h(0.0, 0.0, 100.0, 100.0);

        Scraen {
            fbo: frame_buffer,
            fbo_resolution,

            window_transform: Affine2::from_scale_angle_translation(
                draw_rect.wh() / fbo_rect.wh(),
                0.0,
                draw_rect.xy(),
            ),

            scraen_resolution,
            scraen_texture: texture,

            eye_open_percent: (0.1),

            eye_r: fbo_rect.h() / 4.0,
            screen_center: vec2(0.0, 0.0),
            eye_xy: vec2(0.0, 0.0),

            fbo_rect,
            draw_rect,

            eye_rt: vec2(0.0, 0.0),

            target: vec2(0.0, 0.0),

            blink_ease: EaseBlink::new(1.0),
        }
    }

    pub fn update(&mut self, app: &App, target: Point2, time: f32, window: Rect) {
        let smooth = self.target - target;

        self.target = self.target - smooth * 0.6;
        let target = self.target;
        self.eye_xy = self.window_transform.inverse().transform_point2(target);

        // let eye = self.draw_rect.xy();

        // let dist = eye.distance(target) / 2.0;

        // let angle = Scraen::angle(target, eye);

        // let transfrom = Affine2::from_scale_angle_translation(
        //     window.wh() / self.draw_rect.wh(),
        //     0.0,
        //     self.draw_rect.xy(),
        // );
        // self.eye_xy = ;
        // * self.fbo_rect.wh()

        // self.eye_rt = vec2(30.0, angle);
        // self.eye_xy = Scraen::xy_from_rt(self.eye_rt);

        // if random_range(0, 100) > 90 {
        //     self.blink_ease.start_ease()
        // }
        // self.blink_ease.update(time);
    }
    pub fn angle(a: Point2, b: Point2) -> f32 {
        (a - b).normalize().angle()
    }
    pub fn xy_from_rt(rt: Point2) -> Point2 {
        let radius = rt.x;
        let theta = rt.y;

        let x = radius * theta.cos();
        let y = radius * theta.sin();

        vec2(x, y)
    }

    pub fn draw_eye(&self) {
        let draw = &self.fbo.draw();
        draw.background().color(BLACK);
        let rect_height = self.eye_r * self.blink_ease.val;

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
                    FilterType::Nearest,
                ),
            );
        }
    }

    pub fn draw_to_frame(&self, draw: &Draw) {
        let t = self.window_transform;

        draw.texture(&self.scraen_texture)
            .xy(t.transform_point2(self.fbo_rect.xy()))
            .wh(t.transform_vector2(self.fbo_rect.wh()));
    }

    pub fn serial_packet(&self) -> Option<Vec<u8>> {
        if let Ok(image) = self.fbo.image_buffer.try_lock() {
            let small_img = image
                .resize_exact(
                    self.scraen_resolution.0,
                    self.scraen_resolution.1,
                    FilterType::Gaussian,
                )
                .rotate90();

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
// trait EaseExt {
//     fn blink_ease(&self, d: f32) -> f32 {
//         0.0
//     }
// }
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
