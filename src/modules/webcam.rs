use image::DynamicImage;
use nannou::prelude::*;
use nannou::{App, Draw};
use nannou_wgpu::Texture;

mod camera;
use camera::Camera;
mod detector;
use detector::Detector;

pub struct Webcam {
    cam: Camera,
    detector: Detector,
    camera_texture: Texture,
    // pub video_size: Vec2,
    // pub texture: Texture,
}
impl Webcam {
    pub fn new(app: &App) -> Webcam {
        let cam = Camera::new(1);

        let empty = &DynamicImage::new_rgb8(cam.w(), cam.h());

        Webcam {
            cam,
            camera_texture: Texture::from_image::<&App>(app, empty),
            detector: Detector::new(),
        }
    }
    pub fn update(&mut self, app: &App) {
        self.cam.get_frame();

        //render camera
        let width = self.cam.w();
        let height = self.cam.h();

        let data = &self.cam.data;
        if let Some(data) = &data {
            let image =
                image::ImageBuffer::from_fn(width, height, |x, y| {
                    let pixel = data[y as usize][(width - x - 1) as usize];
                    image::Rgb([
                        pixel[2] as u8,
                        pixel[1] as u8,
                        pixel[0] as u8,
                    ])
                });
            let img = DynamicImage::ImageRgb8(image.clone());
            self.camera_texture = Texture::from_image::<&App>(app, &img);
        }
        //face stuff
        self.detector.update_faces(&self.cam.img);
    }
    pub fn draw(&self, draw: &Draw) {
        //draw camera
        draw.texture(&self.camera_texture)
            .wh(self.webcam_rect().wh());
        //draw faces
        for face in &self.detector.faces {
            let w = face.width as f32 * 4.0;
            let h = face.height as f32 * 4.0;
            let x = face.x as f32 * 4.0 + w / 2.0;
            let y = face.y as f32 * 4.0 + h / 2.0;

            // face.x as f32 * 4.0 - offset_x,
            // face.y as f32 * 4.0 - offset_y,

            let offset_x = self.cam.w() as f32 / 2.0 * -1.0;
            let offset_y = self.cam.h() as f32 / 2.0;

            //convert face to rect
            let face = Rect::from_x_y_w_h(x + offset_x, y, w, h);

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
