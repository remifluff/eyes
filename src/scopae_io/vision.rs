// use std::sync::{Arc, Mutex};
// use std::thread;

// // use nokhwa::{query, Camera, CameraFormat, FrameFormat, ThreadedCamera};
// // use rustface::FaceInfo;

// use image::{GenericImageView, ImageBuffer, Rgb};
// use nannou::draw::primitive::rect;
// use nannou::image::{DynamicImage, GrayImage};
// use nannou::lyon::geom::euclid::point2;
// use nannou::lyon::math::Point;
// use nannou::prelude::*;

// use nokhwa::{Camera, CameraFormat, FrameFormat, ThreadedCamera};
// use rustface::{Detector, FaceInfo, ImageData};
// use std::sync::mpsc::{self, channel, Receiver, Sender};
// use wgpu::{TextueSnapshot, Texture};

// const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";
// const CAMERA_WH: (f32, f32) = (320.0, 240.0);

// pub struct Vision {
//     camera: Option<ThreadedCamera>,
//     image: DynamicImage,
//     texture: Texture,

//     // detector: Arc<Mutex<AsyncDetector>>
//     faces: Arc<Mutex<Vec<Rect>>>,
//     downscale_factor: f32,

//     scale_factor: Point2,
//     wh: Point2,
// }

// impl Vision {
//     pub fn new(app: &App) -> Vision {
//         let image = image::open("model/faces.jpg").unwrap();
//         let (w, h) = CAMERA_WH;

//         let mut wh = Point2::new(w, h);

//         let camera = if let Ok(mut c) = ThreadedCamera::new(
//             0,
//             Some(CameraFormat::new_from(
//                 w as u32,
//                 h as u32,
//                 FrameFormat::MJPEG,
//                 30,
//             )), // format
//         ) {
//             c.open_stream(callback);
//             Some(c)
//         } else {
//             let dim = image.dimensions();
//             wh = vec2(dim.0 as f32, dim.1 as f32);
//             None
//         };

//         let texture = Texture::from_image::<&App>(app, &image);

//         let mut detector_raw = rustface::create_detector(MODEL_PATH).unwrap();

//         detector_raw.set_min_face_size(40);
//         detector_raw.set_score_thresh(2.0);
//         detector_raw.set_pyramid_scale_factor(0.1);
//         detector_raw.set_slide_window_step(4, 4);

//         let detector = AsyncDetector {
//             inner: detector_raw,
//         };

//         Vision {
//             image,
//             texture,
//             camera,
//             downscale_factor: 1.0,
//             detector: Arc::new(Mutex::new(detector)),
//             faces: Arc::new(Mutex::new(Vec::new())),

//             scale_factor: Point2::new(0.0, 0.0),
//             wh,
//         }
//     }
//     pub fn initialize(&self) {}

//     pub fn update_faces(&mut self) {
//         let detector = Arc::clone(&self.detector);
//         let faces = Arc::clone(&self.faces);
//         let image = &self.image;
//         let m = image.clone();

//         let handle = thread::spawn(move || {
//             if let Ok(mut dectector) = detector.lock() {
//                 *faces.lock().unwrap() = dectector.detect(&m);
//             }
//         });
//     }
//     pub fn update_camera(&mut self, app: &App, screen: Rect) {
//         self.scale_factor = screen.wh() / self.wh;
//         self.scale_factor = Point2::from([self.scale_factor.max_element(); 2]);
// <<<<<<< HEAD
//         // self.scale_factor = vec2(1, 0.2);
//         // if let Ok(face) = self.faces.lock() {
//         //     iter()
//         // }
//         //
// =======

// >>>>>>> 7c65c0e987f48d6d5dabee4ea630ec978218182d
//         if let Some(cam) = &mut self.camera {
//             if let Ok(img) = &mut cam.poll_frame() {
//                 let (thumb_w, thumb_h) = (
//                     (self.wh.x * self.downscale_factor) as u32,
//                     (self.wh.y * self.downscale_factor) as u32,
//                 );
//                 self.image = DynamicImage::ImageRgb8(img.clone()).thumbnail(thumb_w, thumb_h);
//                 self.texture = Texture::from_image::<&App>(app, &self.image);
//                 unsafe {
//                     CAMERA_READY = false;
//                 }
//             };
//         }
//     }

//     pub fn draw_camera(&self, draw: &Draw, offset: Point2) {
//         draw.texture(&self.texture)
//             .wh(self.wh * self.scale_factor * vec2(-1.0, 1.0))
//             .xy(vec2(0.0, 0.0));
//     }

//     pub fn draw_face(&self, draw: &Draw, screen: Rect, offset: Point2) {
//         if let Ok(faces) = self.faces.lock() {
//             let offset_pos = self.wh;

//             for face in faces.iter() {
//                 // let xy = (face.xy() + face.wh() * 0.5 - self.wh * 0.5)
//                 //     * vec2(1.0, -1.0)
//                 //     * self.scale_factor;

//                 draw.rect()
//                     .wh(face.wh() * self.scale_factor)
//                     .xy(face.xy() * self.scale_factor)
//                     .color(BLUE);
//             }
//         }
//     }
// }

// fn callback(image: nannou::image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
//     unsafe {
//         CAMERA_READY = true;
//     }
//     // println!("{}x{} {}", image.width(), image.height(), image.len());
// }

// pub fn update_number(counter: Arc<Mutex<i32>>) {
//     let handle = thread::spawn(move || {
//         let mut num = counter.lock().unwrap();

//         *num += 1;
//     });
// }

// pub struct AsyncDetector {
//     inner: Box<dyn Detector>,
// }

// impl AsyncDetector {
//     pub fn detect(&mut self, image: &DynamicImage) -> Vec<Rect> {
//         let gray = image.to_luma8();
//         let (w, h) = gray.dimensions();

//         let image = ImageData::new(&gray, w, h);
//         self.inner
//             .detect(&image)
//             .to_owned()
//             .iter()
//             .map(|face| {
//                 let image_wh = vec2(w as f32, h as f32);
//                 let bbox = face.bbox();
//                 let wh = vec2(bbox.width() as f32, bbox.height() as f32);
//                 let mut xy = vec2(bbox.x() as f32, bbox.y() as f32);
//                 xy = -xy - wh / 2.0 + image_wh / 2.0;

//                 Rect::from_xy_wh(xy, wh)
//             })
//             .collect()
//     }
// }
// unsafe impl Send for AsyncDetector {}

// pub fn update_faces(
//     detector: Arc<Mutex<AsyncDetector>>,
//     faces: Arc<Mutex<Vec<Rect>>>,
//     image: &DynamicImage,
// ) {
//     let m = image.clone();
//     let handle = thread::spawn(move || {
//         if let Ok(mut dectector) = detector.lock() {
//             *faces.lock().unwrap() = dectector.detect(&m);
//         }

//         // // faces
//     });
// }
