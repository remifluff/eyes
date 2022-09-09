use crate::{screen, Connection, Fbo, Model, CAMERA_READY, PORT};
use image::{ImageBuffer, Rgb};
use nannou::draw::primitive::rect;
use nannou::image::{DynamicImage, GrayImage};
use nannou::prelude::*;
use nokhwa::{Camera, CameraFormat, FrameFormat, ThreadedCamera};
use rustface::{Detector, FaceInfo, ImageData};
use wgpu::{TextueSnapshot, Texture};

pub struct Vision {
    camera: Option<ThreadedCamera>,
    image: DynamicImage,
    texture: Texture,

    detector: Box<dyn Detector>,
    faces: Vec<FaceInfo>,
    downscale_factor: f32,
    wh: Point2,
    w: u32,
    h: u32,
}

impl Vision {
    pub fn new(app: &App, model_path: &str, (w, h): (f32, f32)) -> Vision {
        // let mut threaded = ThreadedCamera::new(0, None).unwrap();
        let wh = Point2::new(w, h);

        let w = w as u32;
        let h = h as u32;

        let camera = if let Ok(mut c) = ThreadedCamera::new(
            0,
            Some(CameraFormat::new_from(w, h, FrameFormat::MJPEG, 30)), // format
        ) {
            c.open_stream(callback);
            Some(c)
        } else {
            None
        };

        let image = image::open("model/faces.jpg").unwrap();
        // let image = DynamicImage::new_rgb8(w, h);
        let texture = Texture::from_image::<&App>(app, &image);

        let mut detector = rustface::create_detector(model_path).unwrap();

        detector.set_min_face_size(40);
        detector.set_score_thresh(2.0);
        detector.set_pyramid_scale_factor(0.1);
        detector.set_slide_window_step(4, 4);

        Vision {
            image,
            faces: Vec::new(),
            texture,
            camera,
            downscale_factor: 1.0,
            detector,
            wh,
            w,
            h,
        }
    }
    pub fn initialize(&self) {}
    pub fn update_faces(&mut self, app: &App) {
        self.faces = {
            let detector: &mut dyn Detector = &mut *self.detector;
            let gray = self.image.clone().to_luma8();
            let (w, h) = gray.dimensions();
            let mut image = ImageData::new(&gray, w, h);
            let faces = detector.detect(&mut image);
            faces
        };
    }
    pub fn update_camera(&mut self, app: &App) {
        if let Some(cam) = &mut self.camera {
            if let Ok(img) = &mut cam.poll_frame() {
                let (thumb_w, thumb_h) = (
                    (self.w as f32 * self.downscale_factor) as u32,
                    (self.h as f32 * self.downscale_factor) as u32,
                );
                self.image = DynamicImage::ImageRgb8(img.clone()).thumbnail(thumb_w, thumb_h);
                self.texture = Texture::from_image::<&App>(app, &self.image);
                unsafe {
                    CAMERA_READY = false;
                }
            };
        }
    }

    pub fn draw_camera(&self, draw: &Draw, screen: Rect) {
        //  screem.w_h()

        draw.texture(&self.texture).wh(screen.wh());
    }

    pub fn draw_face(&self, draw: &Draw, screen: Rect) {
        for face in self.faces.iter() {
            let face_rect = face.reshape_face(screen);
            draw.rect().wh(face_rect.wh()).xy(face_rect.xy());
        }
    }
}

fn callback(image: nannou::image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    unsafe {
        CAMERA_READY = true;
    }
    // println!("{}x{} {}", image.width(), image.height(), image.len());
}
trait TransformExt {
    fn reshape_face(&self, screen: Rect) -> Rect {
        Rect::from_w_h(0.0, 0.0)
    }
}

impl TransformExt for FaceInfo {
    fn reshape_face(&self, screen: Rect) -> Rect {
        let middle = screen.x();
        let bbox = self.bbox();
        let (mut x, mut y, mut w, mut h) = (
            bbox.x() as f32,
            bbox.y() as f32,
            bbox.width() as f32,
            bbox.height() as f32,
        );

        x = -(x - middle) + middle;
        // y = -(y - middle) + middle;
        Rect::from_x_y_w_h(x * 2.0 + 500.0, y * 2.0 - 200.0, w * 2.0, h * 2.0)
    }
}
