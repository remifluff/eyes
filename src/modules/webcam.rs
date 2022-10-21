use image::DynamicImage;
use nannou::prelude::*;
use nannou::{App, Draw};
use nannou_wgpu::Texture;

mod camera;
use camera::Camera;
mod detector;
use detector::Detector;
use DynamicImage::*;

use crate::settings::Orientation::*;
use crate::settings::*;

pub struct Webcam {
    cam: Camera,
    detector: Detector,
    camera_texture: Texture,
    // pub video_size: Vec2,
    // pub texture: Texture,
    downscale_factor: f32,
    draw_transformation: Affine2,
    orientation: Orientation,
    cam_rect: Rect,

    pub target: Option<Rect>,
}
impl Webcam {
    pub fn new(
        app: &App,
        index: usize,
        draw_rect: Rect,
        cam_w: u32,
        cam_h: u32,
        orientation: Orientation,
    ) -> Webcam {
        let cam = Camera::new(index, cam_w, cam_h);

        let cam_rect = match orientation {
            Orientation::Portrait => {
                Rect::from_x_y_w_h(0.0, 0.0, cam_h as f32, cam_w as f32)
            }
            Orientation::Landscape => {
                Rect::from_x_y_w_h(0.0, 0.0, cam_w as f32, cam_h as f32)
            }
        };

        let empty = &DynamicImage::new_rgb8(
            cam_rect.w() as u32,
            cam_rect.h() as u32,
        );

        get_transformation(cam_rect, draw_rect);

        Webcam {
            cam,

            camera_texture: Texture::from_image::<&App>(app, empty),
            detector: Detector::new(DOWNSCALE_FACTOR),
            downscale_factor: DOWNSCALE_FACTOR,
            draw_transformation: get_transformation(cam_rect, draw_rect),

            orientation,
            cam_rect,
            target: None,
        }
    }
    pub fn update(&mut self, app: &App) {
        self.cam.get_frame();

        //render camera

        if let Some(data) = &self.cam.data {
            let width = self.cam.w();
            let height = self.cam.h();

            let image =
                image::ImageBuffer::from_fn(width, height, |x, y| {
                    let pixel = data[y as usize][(width - x - 1) as usize];
                    image::Rgb([
                        pixel[2] as u8,
                        pixel[1] as u8,
                        pixel[0] as u8,
                    ])
                });

            let img = match &self.orientation {
                Portrait => ImageRgb8(image.clone()).rotate90(),
                Landscape => ImageRgb8(image.clone()),
            };
            // let img = img.thumbnail(width / 10, height / 10);

            self.camera_texture = Texture::from_image::<&App>(app, &img);
        }
        //face stuff
        self.detector.update_faces(
            &self.cam.get_img(&self.orientation),
            &self.orientation,
        );

        if let Some(face) = self.detector.biggest_rect {
            let x = face.0;
            let y = face.1;
            let w = face.2;
            let h = face.3;
            let offset_x = self.cam.w() as f32 / 2.0;
            let offset_y = self.cam.h() as f32 / 2.0;

            // if h > 1.0 {
            self.target = Some(
                Rect::from_x_y_w_h(offset_x - x, offset_y - y, w, h)
                    .transform(self.draw_transformation),
            );
        } else {
            self.target = None;
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let draw_rect = self.cam_rect.transform(self.draw_transformation);

        //draw camera
        draw.texture(&self.camera_texture)
            .wh(draw_rect.wh())
            .xy(draw_rect.xy());
        //draw faces

        for face in &self.detector.faces {
            let x = face.0;
            let y = face.1;
            let w = face.2;
            let h = face.3;

            // face.x as f32 * 4.0 - offset_x,
            // face.y as f32 * 4.0 - offset_y,

            let offset_x = self.cam.w() as f32 / 2.0;
            let offset_y = self.cam.h() as f32 / 2.0;

            //convert face to rect
            let face =
                Rect::from_x_y_w_h(offset_x - x, offset_y - y, w, h)
                    .transform(self.draw_transformation);

            //draw face
            draw.rect().color(WHITE).xy(face.xy()).wh(face.wh());
        }
    }

    pub fn webcam_rect(&self) -> Rect {
        Rect::from_x_y_w_h(
            0.0,
            0.0,
            self.cam.w() as f32,
            self.cam.h() as f32,
        )
    }
}

fn get_transformation(from_rect: Rect, to_rect: Rect) -> Affine2 {
    let scale_change = to_rect.wh() / from_rect.wh();
    let position_change = to_rect.xy() - from_rect.xy();
    // + (-from_rect.wh() / 2.0) * (to_rect.wh() / from_rect.wh());

    Affine2::from_scale_angle_translation(
        scale_change,
        0.0,
        position_change,
    )
}

trait TranformExt {
    fn transform(&self, t: Affine2) -> Rect {
        Rect::from_w_h(0.0, 0.0)
    }
}

impl TranformExt for Rect {
    fn transform(&self, t: Affine2) -> Rect {
        Rect::from_xy_wh(
            t.transform_point2(self.xy()),
            t.transform_vector2(self.wh()),
        )
    }
}
