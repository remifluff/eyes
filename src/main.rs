#![allow(dead_code)]
#![allow(unused_imports)]
use chrono::{TimeZone, Utc};
use core::num;
use nannou::draw;
use nannou::draw::primitive::rect;
use nannou::lyon::geom::euclid::SideOffsets2D;
use nannou::{draw::background::new, ease, prelude::*, wgpu::ToTextureView};
use nannou_egui::egui::Slider;
use nokhwa::{query, Camera, CameraFormat, FrameFormat, Resolution, ThreadedCamera};
use time::macros::{date, datetime};

use osc::Message;
use rustface::FaceInfo;
use std::time::Instant;
use std::time::{Duration, SystemTime};
use std::{os::unix::prelude::DirEntryExt, string};
use time::Weekday::Wednesday;
use time::{Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};
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

mod walk;
use walk::Walk;

pub use serial2::SerialPort;

// const PORT_NAME: &str = "/dev/ttyprintk";
const PORT_NAME: &str = "/dev/ttyACM0";

use nannou::image::DynamicImage::ImageRgb8;
use nannou_osc as osc;

const SCRAEN_SCALE: f32 = 10.0 * SCALE;

const SCRAENS: [ScraenDim; 4] = [
    ScraenDim {
        rez: 4,
        xy: (466.0, -123.0),
        wh: (4.0, 4.0),
    },
    ScraenDim {
        rez: 16,
        xy: (102.0, -212.0),
        wh: (16.0, 16.0),
    },
    ScraenDim {
        rez: 8,
        xy: (38.0, 92.0),
        wh: (8.0, 8.0),
    },
    ScraenDim {
        rez: 12,
        xy: (453.0, 124.0),
        wh: (12.0, 12.0),
    },
];

const OSC_PORT: u16 = 8338;

pub struct ScraenDim {
    rez: u32,
    xy: (f32, f32),
    wh: (f32, f32),
}

pub struct Settings {
    min_radius: f32,
    max_radius: f32,
    circle_count: usize,
}

fn main() {
    nannou::app(model).update(update).run();
}
const SHOWDEBUG: bool = true;
pub struct Model {
    scraens: Vec<Scraen>,
    vision: Vision,
    // vision2: Vision,
    ui: UI,
    port: Connection,

    face_cam_rect: Rect,
    street_cam_rect: Rect,
    target: Vec2,
    walk_x: Walk,
    walk_y: Walk,
}
const SCALE: f32 = 2.5;

const CAMERA_WH: (u32, u32) = (320, 240);

const WIDTH: f32 = 240.0 * 2.0;
const HEIGHT: f32 = 360.0 * 1.0;

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size((WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let win_rect = window.rect();

    let face_cam_rect = Rect::from_corners(win_rect.top_left(), win_rect.mid_bottom());
    let street_cam_rect = Rect::from_corners(win_rect.mid_top(), win_rect.bottom_right());

    let mut screen = Vec::new();
    for scraen_dim in SCRAENS {
        screen.push(Scraen::new(app, scraen_dim, face_cam_rect));
    }

    let mut port = Connection::new(PORT_NAME, false);
    port.open_port();
    Connection::print_avaliable_ports();

    let write_timer = Timer::start_new(app.time, 0.0001);

    let vision_timer = Timer::start_new(app.time, 0.1);

    let rez: (u32, u32) = (12, 12);

    let mut vision = Vision::new(app, CAMERA_WH, [(0, face_cam_rect), (2, street_cam_rect)]);

    vision.update_camera(app, face_cam_rect);

    Model {
        scraens: screen,
        vision,
        ui: UI::new(&window),
        port,
        face_cam_rect,
        street_cam_rect,
        target: vec2(0.0, 0.0),
        walk_x: Walk::new(43324),
        walk_y: Walk::new(621034),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    let win = app.window_rect();

    model.vision.update_camera(app, model.face_cam_rect);
    model.vision.update_faces();

    if let Some(t) = model.vision.get_target() {
        model.target = t;
    };

    // model.target = app.mouse.position();
    let walk = vec2(model.walk_x.val(), model.walk_y.val()) - model.face_cam_rect.xy();
    model.target = walk;

    model.walk_x.update();
    model.walk_y.update();

    model.port.write(vec![255]);
    for screen in &mut model.scraens {
        screen.draw_eye();
        screen.render_texture(&app);

        screen.update(&app, model.target, time.into());
        if let Some(buf) = screen.serial_packet() {
            model.port.write(buf);
        }
    }
    model.port.write(vec![254]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.vision.draw_camera(&draw);
    model.vision.draw_face(&draw, model.face_cam_rect);

    if SHOWDEBUG {
        for screen in &model.scraens {
            screen.draw_to_frame(&draw);
        }
    }

    let target2 = app.mouse.position();
    // let target = model.vision.biggest_face.xy();

    use chrono;

    fn main() {
        println!("{:?}", chrono::offset::Local::now());
        println!("{:?}", chrono::offset::Utc::now());
    }

    let dt = chrono::offset::Local::now();
    dt.format("%Y-%m-%d %H:%M:%S");
    // font::collection_from_file( model/Futura.ttc)
    let walk = vec2(model.walk_x.val(), model.walk_y.val()) - model.face_cam_rect.xy();

    if SHOWDEBUG {
        draw.ellipse().xy(walk).radius(30.0).color(GREY);
    }

    draw.text(format!("local time: {}", dt.format("%H:%M:%S:%f")).as_str())
        .color(WHITE)
        .font_size(24)
        .w_h(800.0, 10.0)
        .x_y(0.0, -370.0);
    draw.to_frame(app, &frame).unwrap();
}
