#![allow(dead_code)]
#![allow(unused_imports)]
use core::num;
use std::time::Instant;
use std::{os::unix::prelude::DirEntryExt, string, time::Duration};

use nannou::draw;
use nannou::draw::primitive::rect;
use nannou::lyon::geom::euclid::SideOffsets2D;
use nannou::{draw::background::new, ease, prelude::*, wgpu::ToTextureView};
use nokhwa::{query, Camera, CameraFormat, FrameFormat, ThreadedCamera};
use osc::Message;
use rustface::FaceInfo;
use wgpu::Texture;

pub mod connection;
use crate::fbo::Fbo;
mod fbo;
use crate::connection::Connection;

mod screen;
use screen::Screen;

mod eye;
use eye::Eye;

mod vision;
use vision::Vision;

pub use serial2::SerialPort;

const PORT_NAME: &str = "/dev/cu.usbmodem105641701";
static mut PORT: Connection = Connection::new(PORT_NAME, false);
static mut FACES: Vec<FaceInfo> = Vec::new();

static mut CAMERA_READY: bool = false;

use nannou::image::DynamicImage::ImageRgb8;
use nannou_osc as osc;

const OSC_PORT: u16 = 8338;
const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";
const CAMERA_WH: (f32, f32) = (320.0, 240.0);

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

pub struct Model {
    eye: Eye,
    write_timer: Timer,
    vision_timer: Timer,
    screen: [Screen; 2],
    vision: Vision,
    vision2: Vision,
}

fn model(app: &App) -> Model {
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
    let vision_timer = Timer::start_new(app.time, 0.1);

    query().iter().for_each(|cam| println!("{:?}", cam));

    let screen = [
        Screen::new(app, Point2::new(12.0, 12.0)),
        Screen::new(app, Point2::new(8.0, 8.0)),
    ];
    let mut vision = Vision::new(app, MODEL_PATH, CAMERA_WH, 0);
    let mut vision2 = Vision::new(app, MODEL_PATH, CAMERA_WH, 2);

    vision.initialize();
    vision2.initialize();

    let win = app.window_rect();

    vision.update_camera(app, win);
    vision2.update_camera(app, win);

    Model {
        eye,
        write_timer,
        screen,
        vision,
        vision2,
        vision_timer,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    model.vision.update_camera(app, win);
    model.vision2.update_camera(app, win);

    let t = app.time;

    model.eye.set_center(app.mouse.position());
    model.eye.update_openess(t);
    if model.write_timer.check(t) {}

    if model.vision_timer.check(t) {
        unsafe {
            model.vision.update_faces(app);
            model.vision2.update_faces(app);
        }
    }

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

    // let rect = frame.rect().pad(10.0);

    let win = app.window_rect();

    // draw.texture(&m.screen.fbo.texture).w_h(600.0, 600.0);
    // draw.texture(&m.screen.fbo.texture).w_h(20.0, 20.0);

    for screen in &model.screen {
        screen.draw_to_frame(app);
    }

    let offset = vec2(100.0, 100.0);
    let offset2 = vec2(-100.0, -100.0);

    model.vision.draw_camera(&draw, offset);
    model.vision.draw_face(&draw, win, offset);

    model.vision2.draw_camera(&draw, offset2);
    model.vision2.draw_face(&draw, win, offset2);

    draw.to_frame(app, &frame).unwrap();
}

struct Timer {
    duration_sec: f32,
    last: f32,
}

impl Timer {
    fn start_new(time: f32, duration_sec: f32) -> Timer {
        Timer {
            duration_sec,
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
