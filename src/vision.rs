use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{scraen, Connection, Model};
use image::{GenericImageView, ImageBuffer, Rgb};
use nannou::draw::primitive::rect;
use nannou::image::{DynamicImage, GrayImage};
use nannou::lyon::geom::euclid::point2;
use nannou::lyon::math::Point;
use nannou::{frame, prelude::*};
use nokhwa::{Camera, CameraFormat, FrameFormat, ThreadedCamera};
use rustface::{Detector, FaceInfo, ImageData};
use std::sync::mpsc::{self, channel, Receiver, Sender};
use wgpu::{TextueSnapshot, Texture};

const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";
const CAMERA_WH: (f32, f32) = (320.0, 240.0);

enum Frame {
    Empty(),
    Unprocessd(DynamicImage),
    Processed(DynamicImage),
}

pub struct Cam {
    backend: Camera,
    frame: Frame,
    texture: Option<Texture>,
}
pub struct Vision {
    cameras: [Arc<Mutex<Cam>>; 1],

    detector: Arc<Mutex<AsyncDetector>>,

    faces: Arc<Mutex<Vec<Rect>>>,
    downscale_factor: f32,

    scale_factor: Point2,
    wh: Point2,
}

impl Vision {
    pub fn new(app: &App, face_cam: usize, street_cam: usize) -> Vision {
        // let image = image::open("model/faces.jpg").unwrap();
        let (w, h) = CAMERA_WH;

        let mut wh = Point2::new(w, h);
        let format = CameraFormat::new_from(w as u32, h as u32, FrameFormat::MJPEG, 30);

        let face_cam = Arc::new(Mutex::new(Cam {
            backend: Camera::new(face_cam, Some(format)).unwrap(),
            frame: Frame::Empty(),
            texture: None,
        }));

        let street_cam = Arc::new(Mutex::new(Cam {
            backend: Camera::new(street_cam, Some(format)).unwrap(),
            frame: Frame::Empty(),
            texture: None,
        }));

        let mut detector_raw = rustface::create_detector(MODEL_PATH).unwrap();

        detector_raw.set_min_face_size(40);
        detector_raw.set_score_thresh(2.0);
        detector_raw.set_pyramid_scale_factor(0.1);
        detector_raw.set_slide_window_step(4, 4);

        let detector = AsyncDetector {
            inner: detector_raw,
        };

        Vision {
            // cameras: [face_cam, street_cam],
            cameras: [face_cam],

            detector: Arc::new(Mutex::new(detector)),
            faces: Arc::new(Mutex::new(Vec::new())),
            downscale_factor: 1.0,

            scale_factor: Point2::new(0.0, 0.0),
            wh,
        }
    }

    pub fn initialize_camera(&mut self) {
        for cam in &self.cameras {
            if let Ok(cam) = &mut cam.lock() {
                cam.backend.open_stream().unwrap();
            }
        }
    }

    pub fn update_faces(&mut self) {
        if let Ok(cam) = &mut self.cameras[0].try_lock() {
            if let Frame::Unprocessd(image) = &cam.frame {
                let img = image.clone();

                let detector = Arc::clone(&self.detector);
                let faces = Arc::clone(&self.faces);

                cam.frame = Frame::Processed(img.to_owned());

                let handle = thread::spawn(move || {
                    if let Ok(mut dectector) = detector.lock() {
                        *faces.lock().unwrap() = dectector.detect(&img);
                    }
                });
            };
        }
    }

    pub fn update_camera_drawboxes(&mut self, screen: Rect) {
        self.scale_factor = screen.wh() / self.wh;
        self.scale_factor = Point2::from([self.scale_factor.min_element(); 2]);
    }

    pub fn render_texture(&mut self, app: &App) {
        for cam in &self.cameras {
            if let Ok(cam) = &mut cam.try_lock() {
                if let Frame::Unprocessd(image) = &cam.frame {
                    cam.texture = Some(Texture::from_image::<&App>(app, &image));
                }
            }
        }
    }

    pub fn update_camera(&mut self, app: &App) {
        let cam_ref = [Arc::clone(&self.cameras[0]), Arc::clone(&self.cameras[0])];

        let handle = thread::spawn(move || {
            for cam in cam_ref {
                if let Ok(cam) = &mut cam.lock() {
                    //spawn thread and iterate through cameras captruing frames
                    if let Ok(captured_buffer) = cam.backend.frame() {
                        cam.frame = Frame::Unprocessd(
                            DynamicImage::ImageRgb8(captured_buffer.clone()).clone(),
                        );
                    }
                }
            }
        }); //finish frames
    }

    pub fn draw_camera(&self, draw: &Draw, offset: Point2) {
        for cam in &self.cameras {
            if let Ok(cam) = &mut cam.lock() {
                if let Some(texture) = &cam.texture {
                    draw.texture(texture)
                        .wh(self.wh * self.scale_factor * vec2(-1.0, 1.0))
                        .xy(vec2(0.0, 0.0) + offset);
                }
            }
        }
    }

    pub fn draw_face(&self, draw: &Draw, screen: Rect, offset: Point2) {
        if let Ok(faces) = self.faces.lock() {
            let offset_pos = self.wh;

            for face in faces.iter() {
                draw.rect()
                    .wh(face.wh() * self.scale_factor)
                    .xy(face.xy() * self.scale_factor + offset)
                    .color(BLUE);
            }
        }
    }
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
    });
}

//             *self.faces.lock().unwrap() = dectector.detect(&frame);
//         }
//     });
// }

// pub fn update_number(counter: Arc<Mutex<i32>>) {
//     let handle = thread::spawn(move || {
//         let mut num = counter.lock().unwrap();

//         *num += 1;
//     });
// }

// println!("{}x{} {}", image.width(), image.height(), image.len());

// fn callback(image: nannou::image::ImageBuffer<Rgb<u8>, Vec<u8>>) {

// pub fn update_detector(&mut self, app: &App) {
//     let frame = Arc::clone(&self.cameras[0].frame);

//     let detector = Arc::clone(&self.detector);
//     let faces = Arc::clone(&self.faces);

//     let handle = thread::spawn(move || {

//         *self.faces.lock().unwrap() = dectector.detect(&frame);

//         if let Ok(mut dectector) = self.detector.lock() {
