use std::sync::{Arc, Mutex};
use std::thread;

use crate::CAMERA_WH;
use image::{ImageBuffer, Rgb};
use nannou::image::DynamicImage;
use nannou::prelude::*;
use nokhwa::{CameraFormat, FrameFormat, ThreadedCamera};
use rustface::{Detector, ImageData};
use wgpu::Texture;

const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";
const CAMERA_WH_F32: (f32, f32) = (CAMERA_WH.0 as f32, CAMERA_WH.1 as f32);

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

    pub biggest_face: Rect,

    ping_pong: bool,
}
struct Cam {
    backend: Option<ThreadedCamera>,
    frame: Frame,
    texture: Texture,
    draw_rect: Rect,
    cam_rect: Rect,
    cam_to_screen: Affine2,
}

impl Vision {
    pub fn new(
        app: &App,
        (w, h): (u32, u32),
        settings: [(usize, Rect); 2],
    ) -> Vision {
        let camera_wh = UVec2::new(w, h);
        let format = CameraFormat::new_from(
            CAMERA_WH.0,
            CAMERA_WH.1,
            FrameFormat::MJPEG,
            30,
        );
        let img = &DynamicImage::new_rgb8(CAMERA_WH.0, CAMERA_WH.1);

        let cam_rect =
            Rect::from_corners(Vec2::splat(0.0), camera_wh.as_f32());

        let mut webcams: [Cam; 2] =
            settings.map(|(cam_number, draw_rect)| Cam {
                backend: {
                    if let Ok(backend) =
                        ThreadedCamera::new(cam_number, Some(format))
                    {
                        Some(backend)
                    } else {
                        None
                    }
                },
                frame: Frame::Empty,
                texture: Texture::from_image::<&App>(app, img),
                cam_rect,
                draw_rect,

                cam_to_screen: {
                    Affine2::from_scale_angle_translation(
                        (draw_rect.wh() / cam_rect.wh()),
                        0.0,
                        // -draw_rect.xy() / 2.0,
                        -draw_rect.xy()
                            + (-cam_rect.wh() / 2.0)
                                * (draw_rect.wh() / cam_rect.wh()),
                    )
                },
            });
        for cam in &mut webcams {
            if let Some(backend) = &mut cam.backend {
                backend.open_stream(callback).unwrap();
            }
        }

        let mut detector_raw =
            rustface::create_detector(MODEL_PATH).unwrap();

        detector_raw.set_min_face_size(40);
        detector_raw.set_score_thresh(1.0);
        detector_raw.set_pyramid_scale_factor(0.1);
        detector_raw.set_slide_window_step(4, 4);

        let detector = AsyncDetector {
            inner: detector_raw,
        };

        Vision {
            webcams,
            detector: Arc::new(Mutex::new(detector)),
            faces: Arc::new(Mutex::new(Vec::new())),

            downscale_factor: 1.0,
            scale_factor: Point2::new(0.0, 0.0),
            biggest_face: Rect::from_x_y_w_h(0.0, 0.0, 0.0, 0.0),

            ping_pong: false,
        }
    }
    pub fn initialize(&self) {}

    pub fn update_camera(&mut self, app: &App, screen: Rect) {
        // self.scale_factor = screen.wh() / self.wh;
        // self.scale_factor = Point2::from([self.scale_factor.max_element(); 2]);

        if unsafe { CAMERA_READY } {
            unsafe { CAMERA_READY = false } // println!("{}x{} {}", image.width(), image.height(), image.len());
            let cam = match self.ping_pong {
                true => &mut self.webcams[0],
                false => &mut self.webcams[1],
            };
            self.ping_pong = !self.ping_pong;
            if let Some(backend) = &mut cam.backend {
                if let Ok(img) = &mut backend.poll_frame() {
                    let img =
                        DynamicImage::ImageRgb8(img.clone()).rotate270();
                    cam.texture = Texture::from_image::<&App>(app, &img);
                    cam.frame = Frame::Unprocessd(img);
                }
            }
        }
    }

    pub fn draw_camera(&self, draw: &Draw) {
        for cam in &self.webcams {
            let t = cam.cam_to_screen;
            if let Some(_) = cam.backend {
                draw.texture(&cam.texture)
                    .xy(get_t_xy(cam.cam_rect, t))
                    .wh(get_t_wh(cam.cam_rect, t));
            } else {
                draw.rect()
                    .xy(get_t_xy(cam.cam_rect, t))
                    .wh(get_t_wh(cam.cam_rect, t))
                    .color(WHITE);
            }
        }
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
                    *faces.lock().unwrap() = {
                        //inlined detector
                        let ref mut this = dectector;
                        let image = &frm;
                        let gray = image.to_luma8();
                        let (w, h) = gray.dimensions();

                        let image = ImageData::new(&gray, w, h);
                        this.inner
                            .detect(&image)
                            .to_owned()
                            .iter()
                            .map(|face| {
                                let image_wh = vec2(w as f32, h as f32);
                                let bbox = face.bbox();
                                let wh = vec2(
                                    bbox.width() as f32,
                                    bbox.height() as f32,
                                );
                                let mut xy =
                                    vec2(bbox.x() as f32, bbox.y() as f32);
                                Rect::from_xy_wh(xy * vec2(1.0, -1.0), wh)
                                    .shift(wh + image_wh * vec2(0.0, 0.5))
                            })
                            .collect()
                    };
                }
            });
        }
    }
    pub fn get_target(&mut self) -> Option<Point2> {
        if let Ok(faces) = self.faces.lock() {
            let biggest_face = faces
                .iter()
                .min_by(|a, b| a.h().partial_cmp(&b.h()).unwrap())?;

            self.biggest_face = *biggest_face;
        }
        let t = self.webcams[0].cam_to_screen;

        // .xy(get_t_xy(*face, t))
        // .radius(get_t_wh(*face, t).x)

        Some(t.transform_point2(self.biggest_face.xy()))
    }

    pub fn draw_face(&self, draw: &Draw, screen: Rect) {
        let cam = &self.webcams[0];

        if let Ok(faces) = self.faces.lock() {
            for face in faces.iter() {
                let t = cam.cam_to_screen;

                draw.ellipse()
                    .xy(get_t_xy(*face, t))
                    .radius(get_t_wh(*face, t).x)
                    .color(WHITE);
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
                Rect::from_xy_wh(xy * vec2(1.0, -1.0), wh)
                    .shift(wh + image_wh * vec2(0.0, 0.5))
            })
            .collect()
    }
}
unsafe impl Send for AsyncDetector {}

pub fn get_t_xy(rect: Rect, t: Affine2) -> Point2 {
    t.transform_point2(rect.xy())
}

pub fn get_t_wh(rect: Rect, t: Affine2) -> Point2 {
    t.transform_vector2(rect.wh())
}

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
fn callback(image: ImageBuffer<Rgb<u8>, Vec<u8>>) {
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
