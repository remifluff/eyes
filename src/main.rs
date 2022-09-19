#![allow(dead_code)]
#![allow(unused_imports)]
use core::num;
use std::time::Instant;
use std::{os::unix::prelude::DirEntryExt, string, time::Duration};

use nannou::draw;
use nannou::draw::primitive::rect;
use nannou::lyon::geom::euclid::SideOffsets2D;
use nannou::{draw::background::new, ease, prelude::*, wgpu::ToTextureView};
use nannou_egui::egui::Slider;
use nokhwa::{query, Camera, CameraFormat, FrameFormat, Resolution, ThreadedCamera};
use osc::Message;
use rustface::FaceInfo;
use wgpu::Texture;

pub mod connection;

use crate::connection::Connection;

mod scraen;
use scraen::Scraen;

mod ui;
use ui::UI;

mod vision;
use vision::Vision;

mod timer;
use timer::Timer;

pub use serial2::SerialPort;

// const PORT_NAME: &str = "/dev/ttyprintk";
const PORT_NAME: &str = "/dev/ttyACM0";

const WIDTH: f32 = 640.0 * 2.0;
const HEIGHT: f32 = 360.0 * 2.0;

use nannou::image::DynamicImage::ImageRgb8;
use nannou_osc as osc;

const SCRAENS: [ScraenDim; 4] = [
    ScraenDim {
        rez: 12,
        xy: (20.0, 20.0),
        wh: 100.0,
    },
    ScraenDim {
        rez: 16,
        xy: (-40.0, -220.0),
        wh: 300.0,
    },
    ScraenDim {
        rez: 4,
        xy: (100.0, -80.0),
        wh: 100.0,
    },
    ScraenDim {
        rez: 8,
        xy: (44., 302.),
        wh: 200.0,
    },
];

const OSC_PORT: u16 = 8338;

pub struct ScraenDim {
    rez: u32,
    xy: (f32, f32),
    wh: f32,
}

pub struct Settings {
    min_radius: f32,
    max_radius: f32,
    circle_count: usize,
}

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    scraens: Vec<Scraen>,
    vision: Vision,
    // vision2: Vision,
    ui: UI,
    port: Connection,

    face_cam_rect: Rect,
    street_cam_rect: Rect,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let win = app.window_rect();
    let face_cam_rect = Rect::from_x_y_w_h(-WIDTH / 4.0, 0.0, WIDTH / 2.0, HEIGHT / 2.0);
    let street_cam_rect = Rect::from_x_y_w_h(WIDTH / 4.0, 0.0, WIDTH / 2.0, HEIGHT / 2.0);

    let mut port = Connection::new(PORT_NAME, false);
    port.open_port();
    Connection::print_avaliable_ports();

    let write_timer = Timer::start_new(app.time, 0.0001);

    let vision_timer = Timer::start_new(app.time, 0.1);

    let rez: (u32, u32) = (12, 12);
    let mut screen = Vec::new();

    for scraen_dim in SCRAENS {
        screen.push(Scraen::new(app, scraen_dim));
    }

    let mut vision = Vision::new(app, [(2, face_cam_rect), (0, street_cam_rect)]);
    // let mut vision = Vision::new(app, 0, 2);

    // vision.initialize_camera();

    vision.update_camera(app, face_cam_rect);

    Model {
        scraens: screen,
        vision,
        ui: UI::new(&window),
        port,
        face_cam_rect,
        street_cam_rect,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let t = app.time;

    model.vision.update_camera(app, model.face_cam_rect);
    model.vision.update_faces();
    let win = app.window_rect();

    let target = model.vision.biggest_face.xy();

    model.port.write(vec![255]);
    for screen in &mut model.scraens {
        screen.update(&app, target, t, win);
        screen.draw_eye();
        screen.render_texture(&app);
        if let Some(buf) = screen.serial_packet() {
            model.port.write(buf);
        }
    }
    model.port.write(vec![254]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let offset = vec2(0.0, 0.0);

    model.vision.draw_camera(&draw, offset);
    model.vision.draw_face(&draw, model.face_cam_rect, offset);

    for screen in &model.scraens {
        screen.draw_to_frame(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
