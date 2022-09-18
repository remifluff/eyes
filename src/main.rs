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
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let mut port = Connection::new(PORT_NAME, false);
    port.open_port();
    Connection::print_avaliable_ports();

    let write_timer = Timer::start_new(app.time, 0.0001);

    let vision_timer = Timer::start_new(app.time, 0.1);

    // query().iter().for_each(|cam| println!("{:?}", cam));

    let rez: (u32, u32) = (12, 12);
    let mut screen = Vec::new();
    let rect = Rect::from_x_y_w_h(30.0, 30.0, 200.0, 200.0);
    let rect2 = Rect::from_x_y_w_h(-50.0, -50.0, 20.0, 50.0);

    for scraen_dim in SCRAENS {
        screen.push(Scraen::new(app, scraen_dim));
    }
    // screen.push(ScopaeScreen::new(app, (8, 8)));

    // ScopaeScreen::new(app, Point2::new(8.0, 8.0)),

    let mut vision = Vision::new(app, 0, 2);
    // let mut vision2 = Vision::new(app, 2);

    vision.initialize_camera();
    // vision2.initialize_camera();

    let win = app.window_rect();

    vision.update_camera(app);
    // vision2.update_camera(app, win);

    Model {
        scraens: screen,
        vision,
        // vision2,
        ui: UI::new(&window),
        port,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    // model.ui.update(update);

    let win = app.window_rect();

    model.vision.update_camera(app);
    // model.vision2.update_camera(app, win);

    let t = app.time;

    model.vision.update_faces();
    let win = app.window_rect();

    model.port.write(vec![255]);
    for screen in &mut model.scraens {
        screen.update(&app, app.mouse.position(), t, win);
        screen.draw_eye();
        screen.render_texture(&app);
        if let Some(buf) = screen.serial_packet() {
            model.port.write(buf);
        }
    }
    model.port.write(vec![254]);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // model.screen_box.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // let rect = frame.rect().pad(10.0);

    let win = app.window_rect();

    // draw.texture(&m.screen.fbo.texture).w_h(600.0, 600.0);
    // draw.texture(&m.screen.fbo.texture).w_h(20.0, 20.0);

    let offset = vec2(0.0, 0.0);
    let offset2 = vec2(-100.0, -100.0);

    model.vision.draw_camera(&draw, offset);
    model.vision.draw_face(&draw, win, offset);

    print!("{}", app.mouse.position());
    // model.vision2.draw_camera(&draw, offset2);
    // model.vision2.draw_face(&draw, win, offset2);
    for screen in &model.scraens {
        screen.draw_to_frame(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
    // model.screen_box.draw(&frame);
}
