use std::iter::Flatten;

use crate::{Connection, Model, ScraenDim, SCRAEN_SCALE};
use image::{
    imageops::FilterType, math, DynamicImage, GenericImageView, Pixel,
};
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

const UPSCALE_VAL: u32 = 3;

pub struct Scraen {
    pub fbo: Fbo,
    fbo_resolution: (u32, u32),
    window_transform: Affine2,

    scraen_resolution: (u32, u32),
    scraen_texture: wgpu::Texture,

    eye_open_percent: f32,
    eye_r: f32,
    eye_xy: Point2,
    eye_rt: Point2,

    fbo_rect: Rect,
    draw_rect: Rect,
    target: Vec2,

    blink_ease: EaseBlink,
    webcam_rect: Rect,
}

impl Scraen {
    pub fn new(app: &App, params: ScraenDim, webcam_rect: Rect) -> Scraen {
        let scraen_resolution = (params.rez, params.rez);
        let fbo_resolution =
            (params.rez * UPSCALE_VAL, params.rez * UPSCALE_VAL);

        let fbo_rect = Rect::from_x_y_w_h(
            0.0,
            0.0,
            fbo_resolution.0 as f32,
            fbo_resolution.1 as f32,
        );

        let draw_rect = Rect::from_x_y_w_h(
            params.xy.0,
            params.xy.1,
            params.wh.0 * SCRAEN_SCALE,
            params.wh.1 * -SCRAEN_SCALE,
        );
        // let fbo_resolution = params.(w * 20, h * 20);

        let frame_buffer =
            Fbo::new(app, (fbo_resolution.0, fbo_resolution.1));
        let img = &DynamicImage::new_rgb8(params.rez, params.rez);
        let texture = wgpu::Texture::from_image(app, img);

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
            eye_xy: vec2(0.0, 0.0),

            fbo_rect,
            draw_rect,
            webcam_rect,

            eye_rt: vec2(0.0, 0.0),

            target: vec2(0.0, 0.0),

            blink_ease: EaseBlink::new(1.0),
        }
    }

    pub fn update(&mut self, app: &App, target: Point2, time: f32) {
        self.target = self.update_target(target);
        self.eye_xy = self.set_eye_position(self.target);
        // if random_range(0, 100) > 90 {
        //     self.blink_ease.start_ease()
        // }
        // self.blink_ease.update(time);
    }

    pub fn update_target(&mut self, target: Point2) -> Point2 {
        self.eye_xy =
            self.window_transform.inverse().transform_point2(target);

        let smooth = self.target - target;
        self.target - smooth * 0.6
    }

    pub fn set_eye_position(&self, target: Point2) -> Point2 {
        let screen_center = self.draw_rect.xy();

        let dist = screen_center.distance(target) / 2.0;
        let max_length = self.fbo_rect.wh().min_element() / 2.0;

        let radius = max_length;
        let theta = (target - screen_center).normalize().angle();
        vec2(radius * theta.cos(), radius * theta.sin())
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
            let small_img = image
                .resize_exact(
                    self.scraen_resolution.0,
                    self.scraen_resolution.1,
                    FilterType::Triangle,
                )
                .rotate90();

            // let mut itt = small_img
            //     .clone()
            //     .as_rgba8()?
            //     .enumerate_rows()
            //     .flat_map(|(i, row)| {
            //         let mut mapped_row: Vec<u8> = row
            //             .map(|(x, y, pix)| {
            //                 clamp(
            //                     pix.to_luma().channels()[0],
            //                     0u8,
            //                     200u8,
            //                 )
            //             })
            //             .collect();
            //         if i % 2 == 0 {
            //             mapped_row.reverse();
            //         }

            //         mapped_row.push(0);
            //         mapped_row
            //     })
            //     .collect::<Vec<u8>>();
            // itt.pop();
            // Some(itt)
            None
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
