use std::sync::{Arc, Mutex};
use std::thread;

use crate::{screen, Connection, Fbo, Model, CAMERA_READY, PORT};
use image::{GenericImageView, ImageBuffer, Rgb};
use nannou::draw::primitive::rect;
use nannou::image::{DynamicImage, GrayImage};
use nannou::lyon::geom::euclid::point2;
use nannou::prelude::*;
use nokhwa::{Camera, CameraFormat, FrameFormat, ThreadedCamera};
use rustface::{Detector, FaceInfo, ImageData};
use std::sync::mpsc::{self, channel, Receiver, Sender};
use wgpu::{TextueSnapshot, Texture};

pub struct Vision {
    camera: Option<ThreadedCamera>,
    image: DynamicImage,
    texture: Texture,

    detector: Arc<Mutex<AsyncDetector>>,

    faces: Arc<Mutex<Vec<Rect>>>,
    downscale_factor: f32,

    scale_factor: Point2,
    wh: Point2,
}

impl Vision {
    pub fn new(app: &App, model_path: &str, (w, h): (f32, f32)) -> Vision {
        let image =
            image::open("model/group-portrait-happy-young-casual-people-28761154.jpg").unwrap();

        let mut wh = Point2::new(w, h);

        let camera = if let Ok(mut c) = ThreadedCamera::new(
            0,
            Some(CameraFormat::new_from(
                w as u32,
                h as u32,
                FrameFormat::MJPEG,
                30,
            )), // format
        ) {
            c.open_stream(callback);
            Some(c)
        } else {
            let dim = image.dimensions();
            wh = vec2(dim.0 as f32, dim.1 as f32);
            None
        };

        // let mut detector_raw = rustface::create_detector(model_path).unwrap();

        // let image = DynamicImage::new_rgb8(w, h);
        let texture = Texture::from_image::<&App>(app, &image);

        // let foo = Box::new(Foo { foo: 1 }) as Box<dyn Bar + Send>;

        let mut detector_raw = rustface::create_detector(model_path).unwrap();

        // let foo = Box::new(Foo { foo: 1 }) as Box<dyn Bar + Send>;

        // let (tx, rx): (
        //     Sender<Box<dyn Detector + Send>>,
        //     Receiver<Box<dyn Detector + Send>>,
        // ) = channel();

        // tx

        detector_raw.set_min_face_size(40);
        detector_raw.set_score_thresh(2.0);
        detector_raw.set_pyramid_scale_factor(0.1);
        detector_raw.set_slide_window_step(4, 4);

        let mut detector = AsyncDetector {
            inner: detector_raw,
        };

        Vision {
            image,
            texture,
            camera,
            downscale_factor: 1.0,
            detector: Arc::new(Mutex::new(detector)),
            faces: Arc::new(Mutex::new(Vec::new())),

            scale_factor: Point2::new(0.0, 0.0),
            wh,
        }
    }
    pub fn initialize(&self) {}

    pub fn update_faces(&mut self, app: &App) {
        update_faces(
            Arc::clone(&self.detector),
            Arc::clone(&self.faces),
            &self.image,
        );

        // // let f = Vec::<FaceInfo>::new();
        // // *faces = f;
    }
    pub fn update_camera(&mut self, app: &App, screen: Rect) {
        self.scale_factor = screen.wh() / self.wh;
        self.scale_factor = Point2::from([self.scale_factor.max_element(); 2]);
        // self.scale_factor = vec2(1, 0.2);
        // if let Ok(face) = self.faces.lock() {
        //     iter()
        // }

        if let Some(cam) = &mut self.camera {
            if let Ok(img) = &mut cam.poll_frame() {
                let (thumb_w, thumb_h) = (
                    (self.wh.x * self.downscale_factor) as u32,
                    (self.wh.y * self.downscale_factor) as u32,
                );
                self.image = DynamicImage::ImageRgb8(img.clone()).thumbnail(thumb_w, thumb_h);
                self.texture = Texture::from_image::<&App>(app, &self.image);
                unsafe {
                    CAMERA_READY = false;
                }
            };
        }
    }

    pub fn draw_camera(&self, draw: &Draw, offset: Point2) {
        draw.texture(&self.texture)
            .wh(self.wh * self.scale_factor)
            .xy(vec2(0.0, 0.0));
    }

    pub fn draw_face(&self, draw: &Draw, screen: Rect, offset: Point2) {
        if let Ok(faces) = self.faces.lock() {
            let offset_pos = self.wh;

            for face in faces.iter() {
                let face_center = face.xy() + (face.wh() * vec2(0.5, 0.5));

                let mirror = face_center * vec2(-1.0, 1.0);

                let center = self.wh * vec2(0.5, -0.5);

                let xy = (mirror + center) * self.scale_factor;

                draw.rect().wh(face.wh() * self.scale_factor).xy(xy);
            }
        }
    }
}

fn callback(image: nannou::image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    unsafe {
        CAMERA_READY = true;
    }
    // println!("{}x{} {}", image.width(), image.height(), image.len());
}

pub fn update_number(counter: Arc<Mutex<i32>>) {
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
}

pub struct AsyncDetector {
    inner: Box<dyn Detector>,
}

impl AsyncDetector {
    pub fn detect(&mut self, image: &DynamicImage) -> Vec<Rect> {
        let gray = image.to_luma8();
        let (w, h) = gray.dimensions();
        let image = ImageData::new(&gray, w, h);
        self.inner
            .detect(&image)
            .to_owned()
            .iter()
            .map(|x| rect_from_faceInfo(x))
            .collect()
    }
}
unsafe impl Send for AsyncDetector {}

pub fn update_faces(
    detector: Arc<Mutex<AsyncDetector>>,
    faces: Arc<Mutex<Vec<Rect>>>,
    image: &DynamicImage,
) {
    let m = image.clone();
    let handle = thread::spawn(move || {
        if let Ok(mut dectector) = detector.lock() {
            *faces.lock().unwrap() = dectector.detect(&m);
        }

        // // faces
    });
}

fn rect_from_faceInfo(face: &FaceInfo) -> Rect {
    let bbox = face.bbox();
    let (mut x, mut y, mut w, mut h) = (
        bbox.x() as f32,
        bbox.y() as f32,
        bbox.width() as f32,
        bbox.height() as f32,
    );

    // x = -(x - middle) + middle;
    // y = -(y - middle) + middle;
    Rect::from_x_y_w_h(x, y, w, h)
}
