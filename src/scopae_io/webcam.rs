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

enum FrameStatus {
    Unloaded,
    NewFrameProcessing,
    FrameReady,
    OldFrame,
}

pub struct Webcam {
    camera: Option<ThreadedCamera>,
    pub image: DynamicImage,
    texture: Texture,
    wh: Point2,

    frame_status: Arc<Mutex<FrameStatus>>,
}

impl Webcam {
    pub fn new(app: &App, webcam_dim: (u32, u32)) -> Webcam {
        let image = image::open("model/faces.jpg").unwrap();
        let (w, h) = webcam_dim;

        let mut wh = Point2::new(w as f32, h as f32);

        let camera = if let Ok(mut c) = ThreadedCamera::new(
            0,
            Some(CameraFormat::new_from(w, h, FrameFormat::MJPEG, 30)), // format
        ) {
            c.open_stream(every_frame_callback);
            Some(c)
        } else {
            let dim = image.dimensions();
            wh = vec2(dim.0 as f32, dim.1 as f32);
            None
        };

        let texture = Texture::from_image::<&App>(app, &image);

        let mut detector_raw = rustface::create_detector(MODEL_PATH).unwrap();

        detector_raw.set_min_face_size(40);
        detector_raw.set_score_thresh(2.0);
        detector_raw.set_pyramid_scale_factor(0.1);
        detector_raw.set_slide_window_step(4, 4);

        Webcam {
            image,
            texture,
            camera,
            wh,
            frame_status: Arc::new(Mutex::new(FrameStatus::Unloaded)),
        }
    }

    pub fn initialize(&self) {}

    pub fn update_camera(&mut self, app: &App, screen: Rect) {
        if let Some(cam) = &mut self.camera {
            if let Ok(img) = &mut cam.poll_frame() {
                let (thumb_w, thumb_h) = (self.wh.x as u32, self.wh.y as u32);
                self.image = DynamicImage::ImageRgb8(img.clone()).thumbnail(thumb_w, thumb_h);
                self.texture = Texture::from_image::<&App>(app, &self.image);
            };
        }
    }

    pub fn draw_camera(&self, draw: &Draw, offset: Point2) {
        draw.texture(&self.texture)
            .wh(self.wh * self.scale_factor * vec2(-1.0, 1.0))
            .xy(vec2(0.0, 0.0));
    }
}
fn callback(image: nannou::image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    unsafe {
        CAMERA_READY = true;
    }
}
