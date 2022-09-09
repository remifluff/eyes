#![allow(dead_code)]
#![allow(unused_imports)]
use core::num;
use std::time::Instant;
use std::{os::unix::prelude::DirEntryExt, string, time::Duration};

use nannou::draw;
use nannou::image::{DynamicImage, GrayImage};
use nannou::{draw::background::new, ease, prelude::*, wgpu::ToTextureView};
use nokhwa::{Camera, CameraFormat, FrameFormat};
use osc::Message;
use wgpu::Texture;

pub mod connection;
use crate::fbo::Fbo;
mod fbo;
use crate::connection::Connection;

mod screen;
use screen::Screen;

pub use serial2::SerialPort;

const PORT_NAME: &str = "/dev/cu.usbmodem105641701";
static mut PORT: Connection = Connection::new(PORT_NAME, false);

use nannou::image::DynamicImage::ImageRgb8;
use nannou_osc as osc;

const OSC_PORT: u16 = 8338;
const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";

use rustface::{Detector, FaceInfo, ImageData};

fn main() -> Result<(), ()> {
    nannou::app(model).update(update).simple_window(view).run();
    Ok(())
}

pub struct Model {
    eye: Eye,
    write_timer: Timer,
    screen: [Screen; 2],
    camera: Camera,
    receiver: osc::Receiver,
    received_packets: Vec<(std::net::SocketAddr, osc::Packet)>,
    image: Option<DynamicImage>,
    faces: Vec<FaceInfo>,
}

fn model(app: &App) -> Model {
    let mut camera = Camera::new(
        0,                                                              // index
        Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)), // format
    )
    .unwrap();
    // open stream

    camera.open_stream().unwrap();
    // Bind an `osc::Receiver` to a port.
    // added mroe comments
    let receiver = osc::receiver(OSC_PORT).unwrap();

    // A vec for collecting packets and their source address.
    let received_packets = vec![];

    let eye = Eye {
        x: (0.0),
        y: (0.0),
        r: (3.0),
        open_percent: (0.1),
    };
    unsafe {
        PORT.open_port();
    }
    let write_timer = Timer::start_new(app.time, 0.0001);

    let screen = [
        Screen::new(app, Point2::new(12.0, 12.0)),
        Screen::new(app, Point2::new(8.0, 8.0)),
    ];

    Model {
        eye,
        write_timer,
        screen,
        camera,
        receiver,
        received_packets,
        image: None,
        faces: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.image = Some(ImageRgb8(model.camera.frame().unwrap()));

    let mut detector = rustface::create_detector(MODEL_PATH).unwrap();

    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    // let mut rgb = model.image.unwrap().to_rgb8();
    model.faces = detect_faces(&mut *detector, &model.image.as_ref().unwrap().to_luma8());

    // match rgb.save(OUTPUT_FILE) {
    //     Ok(_) => println!("Saved result to {}", OUTPUT_FILE),
    //     Err(message) => println!("Failed to save result to a file. Reason: {}", message),
    // }
    // println!("{}, {}", frame.width(), frame.height());

    let t = app.time;

    model.eye.set_center(app.mouse.position());
    model.eye.update_openess(t.blink_ease(1.0));
    if model.write_timer.check(t) {}

    for screen in &model.screen {
        let draw = screen.draw();

        draw.background().color(WHITE);
        model.eye.draw(&draw);
        screen.render(app);
        screen.send_to_screen(app);
        screen.draw_to_frame(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for face in &model.faces {
        let bbox = face.bbox();
        draw.rect()
            .w_h(
                f32::from_u32(bbox.width()).unwrap(),
                f32::from_u32(bbox.height()).unwrap(),
            )
            .x_y(
                f32::from_i32(bbox.x()).unwrap(),
                f32::from_i32(bbox.y()).unwrap(),
            );
    }

    let texture = Texture::from_image::<&App>(&app, &model.image.as_ref().unwrap());

    // let rect = frame.rect().pad(10.0);

    let win = app.window_rect();

    // draw.texture(&m.screen.fbo.texture).w_h(600.0, 600.0);
    // draw.texture(&m.screen.fbo.texture).w_h(20.0, 20.0);

    for screen in &model.screen {
        screen.draw_to_frame(app);
    }
    draw.texture(&texture).w_h(600.0, 600.0);

    draw.to_frame(app, &frame).unwrap();
}

fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
    let (width, height) = gray.dimensions();
    let mut image = ImageData::new(gray, width, height);
    let now = Instant::now();
    let faces = detector.detect(&mut image);
    faces
}

fn detect() {}

struct Eye {
    x: f32,
    y: f32,
    r: f32,
    open_percent: f32,
}
impl Eye {
    fn draw(&self, draw: &Draw) {
        let rect_height = self.r * self.open_percent;
        let rect_offset = rect_height / 2.0;

        draw.ellipse().x_y(self.x, self.y).radius(self.r).color(RED);

        draw.rect()
            .x_y(self.x, self.y - self.r + rect_offset)
            .w(self.r * 2.0)
            .h(rect_height)
            .color(BLACK);

        draw.rect()
            .x_y(self.x, self.y + self.r - rect_offset)
            .w(self.r * 2.0)
            .h(rect_height)
            .color(BLACK);
    }

    fn set_center(&mut self, point: Point2) {
        self.x = point.x;
        self.y = point.y;
    }

    fn update_openess(&mut self, percent: f32) {
        self.open_percent = percent;
    }
}

struct Timer {
    duration_sec: f32,
    last: f32,
}

impl Timer {
    fn start_new(time: f32, duration: f32) -> Timer {
        Timer {
            duration_sec: duration,
            last: time,
        }
    }

    fn check(&mut self, t: f32) -> bool {
        if t - self.last > self.duration_sec {
            self.last = t;
            true
        } else {
            false
        }
    }
}

trait EaseExt {
    fn blink_ease(&self, d: f32) -> f32 {
        0.0
    }
}
impl EaseExt for f32 {
    fn blink_ease(&self, d: f32) -> f32 {
        let t = *self % (d * 2.0);
        ease::sine::ease_in_out(t, 0.0, 1.0, d)
    }
}
