// #![allow(dead_code)]
// #![allow(unused_imports)]
use core::num;
use std::{os::unix::prelude::DirEntryExt, string, time::Duration};

use nannou::draw;
use nannou::draw::primitive::rect;
use nannou::{draw::background::new, ease, prelude::*, wgpu::ToTextureView};

mod scopae_io;
use scopae_io::{SerialOutput, Vision};

mod ui;
use ui::UI;

mod scopae_screen;
use scopae_screen::ScopaeScreen;

mod timer;
use timer::Timer;

const PORT_NAME: &str = "/dev/cu.usbmodem105641701";

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

use nannou::image::DynamicImage::ImageRgb8;
use nannou_osc as osc;

const OSC_PORT: u16 = 8338;

pub struct Settings {
    min_radius: f32,
    max_radius: f32,
    circle_count: usize,
}

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    screen: [ScopaeScreen; 2],
    vision: Vision,
    ui: UI,
    serial_port: SerialOutput,
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

    let win = app.window_rect();

    let write_timer = Timer::start_new(app.time, 0.0001);
    let vision_timer = Timer::start_new(app.time, 0.1);

    // query().iter().for_each(|cam| println!("{:?}", cam));

    let screen = [
        ScopaeScreen::new(app, Point2::new(12.0, 12.0)),
        ScopaeScreen::new(app, Point2::new(8.0, 8.0)),
    ];

    let mut vision = Vision::new(app);
    vision.initialize();

    vision.update_camera(app, win);

    Model {
        screen,
        vision,
        ui: UI::new(&window),
        serial_port: SerialOutput::new(PORT_NAME, false),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    // model.ui.update(update);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // let rect = frame.rect().pad(10.0);

    let win = app.window_rect();

    for screen in &model.screen {
        screen.draw_to_frame(&draw);
    }

    let offset = app.mouse.position();

 

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw(&frame);
}
