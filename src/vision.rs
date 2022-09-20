use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{Connection, Model, Settings};
use image::{GenericImageView, ImageBuffer, Rgb};
use nannou::draw::primitive::rect;
use nannou::image::{DynamicImage, GrayImage};
use nannou::lyon::geom::euclid::point2;
use nannou::lyon::math::{rect, Point};
use nannou::prelude::*;
use nokhwa::{Camera, CameraFormat, FrameFormat, ThreadedCamera};
use rustface::{Detector, FaceInfo, ImageData};
use std::sync::mpsc::{self, channel, Receiver, Sender};
use wgpu::{TextueSnapshot, Texture};

const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";
const CAMERA_WH: (f32, f32) = (320.0, 240.0);
static mut CAMERA_READY: bool = false;

pub enum Frame {
    Empty,
    Unprocessd(DynamicImage),
    Processed(DynamicImage),
}
pub struct Vision {
    webcams: [Cam; 2],
    detector: Arc<Mutex<AsyncDetector>>,

    faces: Arc<Mutex<Vec<Rect>>>,
    downscale_factor: f32,

    scale_factor: Point2,
    wh: Point2,

    pub biggest_face: Rect,

    ping_pong: bool,
    camera_space: Rect,
    camspace_to_screenspace: Affine2,
}
struct Cam {
    backend: ThreadedCamera,
    frame: Frame,
    texture: Texture,
    draw_rect: Rect,
}

impl Vision {
    pub fn new(
        app: &App,
        settings: [(usize, Rect); 2], // face_webcam: usize,
                                      // face_draw_rect: Rect,
                                      // street_webcam: usize,
                                      // street_draw_rect: Rect,
    ) -> Vision {
        let (w, h) = CAMERA_WH;

        let mut wh = Point2::new(w, h);

        let mut webcams: [Cam; 2] = settings.map(|(cam_number, draw_rect)| {
            let format = CameraFormat::new_from(w as u32, h as u32, FrameFormat::MJPEG, 30);
            let img = &DynamicImage::new_rgb8(w as u32, h as u32);
            Cam {
                backend: ThreadedCamera::new(cam_number, Some(format)).unwrap(),
                frame: Frame::Empty,
                texture: Texture::from_image::<&App>(app, img),
                draw_rect: draw_rect,
            }
        });
        for cam in &mut webcams {
            cam.backend.open_stream(callback).unwrap();
        }

        let mut detector_raw = rustface::create_detector(MODEL_PATH).unwrap();

        detector_raw.set_min_face_size(40);
        detector_raw.set_score_thresh(0.70);
        detector_raw.set_pyramid_scale_factor(0.1);
        detector_raw.set_slide_window_step(4, 4);

        let detector = AsyncDetector {
            inner: detector_raw,
        };

        let camera_space = Rect::from_x_y_w_h(0.0, 0.0, -w, h);

        Vision {
            camspace_to_screenspace: Affine2::from_scale_angle_translation(
                webcams[1].draw_rect.wh() / camera_space.wh(),
                0.0,
                webcams[1].draw_rect.xy(),
            ),
            webcams,
            camera_space,
            detector: Arc::new(Mutex::new(detector)),
            faces: Arc::new(Mutex::new(Vec::new())),

            downscale_factor: 1.0,
            scale_factor: Point2::new(0.0, 0.0),

            wh,
            biggest_face: Rect::from_x_y_w_h(0.0, 0.0, 0.0, 0.0),

            ping_pong: false,
        }
    }
    pub fn initialize(&self) {}

    pub fn update_camera(&mut self, app: &App, screen: Rect) {
        self.scale_factor = screen.wh() / self.wh;
        self.scale_factor = Point2::from([self.scale_factor.max_element(); 2]);

        if unsafe { CAMERA_READY } {
            unsafe { CAMERA_READY = false } // println!("{}x{} {}", image.width(), image.height(), image.len());
            let cam = match self.ping_pong {
                true => &mut self.webcams[0],
                false => &mut self.webcams[1],
            };
            self.ping_pong = !self.ping_pong;

            if let Ok(img) = &mut cam.backend.poll_frame() {
                let img = DynamicImage::ImageRgb8(img.clone());
                cam.texture = Texture::from_image::<&App>(app, &img.rotate270());
                cam.frame = Frame::Unprocessd(img);
            }
        }
    }

    pub fn draw_camera(&self, draw: &Draw, offset: Point2) {
        for cam in &self.webcams {
            draw.texture(&cam.texture)
                .wh(cam.draw_rect.wh() * vec2(-1.0, 1.0))
                .xy(cam.draw_rect.xy());
        }

        // vec2(0.0, 0.0) + offset
        // self.wh * self.scale_factor * vec2(-1.0, 1.0)

        // face_draw_rect.
        // face_draw_rect.
    }

    pub fn update_faces(&mut self) {
        let cam = &mut self.webcams[0];
        if let Frame::Unprocessd(frame) = &mut cam.frame {
            let detector = Arc::clone(&self.detector);
            let faces = Arc::clone(&self.faces);
            let frm = frame.clone();
            let fr2 = frame.clone();
            cam.frame = Frame::Processed(fr2);
            self.get_target();

            let handle = thread::spawn(move || {
                if let Ok(mut dectector) = detector.lock() {
                    *faces.lock().unwrap() = dectector.detect(&frm);
                }
            });
        }
    }
    pub fn get_target(&mut self) -> Option<()> {
        if let Ok(faces) = self.faces.lock() {
            let biggest_face = faces
                .iter()
                .min_by(|a, b| a.h().partial_cmp(&b.h()).unwrap())?;

            self.biggest_face = *biggest_face;
        }
        Some(())
    }

    pub fn draw_face(&self, draw: &Draw, screen: Rect, offset: Point2) {
        if let Ok(faces) = self.faces.lock() {
            let offset_pos = self.wh;

            for face in faces.iter() {
                let t = self.camspace_to_screenspace;

                // let xy = (face.xy() + face.wh() * 0.5 - self.wh * 0.5)
                //     * vec2(1.0, -1.0)
                //     * self.scale_factor;

                draw.rect()
                    .wh(t.transform_point2(face.wh()))
                    .xy(t.transform_point2(face.xy()))
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

        // // faces
    });
}
fn callback(image: nannou::image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    unsafe { CAMERA_READY = true } // println!("{}x{} {}", image.width(), image.height(), image.len());
}
// fn rect_from_faceInfo(face: &FaceInfo) -> Rect {
//     let bbox = face.bbox();
//     let (mut x, mut y, mut w, mut h) = (
//         bbox.x() as f32,
//         bbox.y() as f32,
//         bbox.width() as f32,
//         bbox.height() as f32,
//     );

//     // x = -(x - middle) + middle;
//     // y = -(y - middle) + middle;
//     Rect::from_x_y_w_h(x, y, w, h)
// }
