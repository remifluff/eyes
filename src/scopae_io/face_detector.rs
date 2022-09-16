use std::sync::{Arc, Mutex};
use std::thread;

// use nokhwa::{query, Camera, CameraFormat, FrameFormat, ThreadedCamera};
// use rustface::FaceInfo;

use image::{GenericImageView, ImageBuffer, Rgb};
use nannou::draw::primitive::rect;
use nannou::image::{DynamicImage, GrayImage};
use nannou::lyon::geom::euclid::point2;
use nannou::lyon::math::Point;
use nannou::prelude::*;

use nokhwa::{Camera, CameraFormat, FrameFormat, ThreadedCamera};
use rustface::{Detector, FaceInfo, ImageData};
use std::sync::mpsc::{self, channel, Receiver, Sender};
use wgpu::{TextueSnapshot, Texture};

const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";
const CAMERA_WH: (f32, f32) = (320.0, 240.0);

pub struct FaceDetector {
    detector: Arc<Mutex<AsyncDetector>>,
    faces: Arc<Mutex<Vec<Rect>>>,
    downscale_factor: f32,
}

impl FaceDetector {
    pub fn new() -> FaceDetector {
        let mut detector_raw = rustface::create_detector(MODEL_PATH).unwrap();

        detector_raw.set_min_face_size(40);
        detector_raw.set_score_thresh(2.0);
        detector_raw.set_pyramid_scale_factor(0.1);
        detector_raw.set_slide_window_step(4, 4);

        let detector = AsyncDetector { inner: detector_raw };

        FaceDetector {
            downscale_factor: 1.0,
            detector: Arc::new(Mutex::new(detector)),
            faces: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn initialize(&self) {}

    pub fn update_faces(&mut self, image: &DynamicImage) {
        let detector = Arc::clone(&self.detector);
        let faces = Arc::clone(&self.faces);

        // let handle = thread::spawn(move || {
        //     if let Ok(mut dectector) = detector.lock() {
        //         *faces.lock().unwrap() = dectector.detect(&image.clone());
        //     }
        // });
    }

    pub fn draw_face(&self, draw: &Draw, screen: Rect, offset: Point2, scale_factor: f32) {
        if let Ok(faces) = self.faces.lock() {
            for face in faces.iter() {
                // let xy = (face.xy() + face.wh() * 0.5 - self.wh * 0.5)
                //     * vec2(1.0, -1.0)
                //     * self.scale_factor;

                draw.rect()
                    .wh(face.wh() * scale_factor)
                    .xy(face.xy() * scale_factor)
                    .color(BLUE);
            }
        }
    }
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
            .map(|face| {
                let image_wh = vec2(w as f32, h as f32);
                let bbox = face.bbox();
                let wh = vec2(bbox.width() as f32, bbox.height() as f32);
                let mut xy = vec2(bbox.x() as f32, bbox.y() as f32);
                xy = -xy - wh / 2.0 + image_wh / 2.0;

                Rect::from_xy_wh(xy, wh)
            })
            .collect()
    }
}
unsafe impl Send for AsyncDetector {}

pub fn update_faces(detector: Arc<Mutex<AsyncDetector>>, faces: Arc<Mutex<Vec<Rect>>>, image: &DynamicImage) {
    let m = image.clone();
    let handle = thread::spawn(move || {
        if let Ok(mut dectector) = detector.lock() {
            *faces.lock().unwrap() = dectector.detect(&m);
        }

        // // faces
    });
}
