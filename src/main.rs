#![allow(dead_code)]
// #![allow(unused_imports)]

use nannou::prelude::*;

pub mod connection;
use crate::connection::Connection;
mod data;
use data::draw_text;
mod scraen;
use scraen::Scraen;
mod vision;
use vision::Vision;
mod timer;
mod walk;
use walk::Walk;
pub mod constants;
use constants::*;

pub use serial2::SerialPort;

pub struct ScraenDim {
    rez: u32,
    xy: (f32, f32),
    wh: (f32, f32),
    rotate: bool,
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
    port: Connection,

    camera_rect: Rect,
    target: Vec2,
    walk_x: Walk,
    walk_y: Walk,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size((WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let win_rect = window.rect();

    let camera_rect = win_rect;

    let mut screen = Vec::new();
    for scraen_dim in SCRAENS {
        screen.push(Scraen::new(app, scraen_dim, camera_rect));
    }

    let mut port = Connection::new(PORT_NAME, true);
    port.open_port();
    Connection::print_avaliable_ports();

    let mut vision = Vision::new(
        app,
        CAMERA_WH,
        camera_rect,
        WEBCAMS_INDEX,
        // [(0, face_cam_rect), (0, street_cam_rect)],
    );

    vision.update(app);

    Model {
        scraens: screen,
        vision,
        port,
        camera_rect,
        target: vec2(0.0, 0.0),
        walk_x: Walk::new(43324),
        walk_y: Walk::new(621034),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;

    let target = model.vision.update(app);

    if let Some(t) = target {
        model.target = t;
    };

    // model.target = app.mouse.position();
    let walk = vec2(model.walk_x.val(), model.walk_y.val()) - model.camera_rect.xy();
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
    model.vision.draw_face(&draw, model.camera_rect);

    if SHOWDEBUG {
        for screen in &model.scraens {
            screen.draw_to_frame(&draw);
        }
    }

    // let target = model.vision.biggest_face.xy();
    let walk = vec2(model.walk_x.val(), model.walk_y.val()) - model.camera_rect.xy();
    if SHOWDEBUG {
        draw.ellipse().xy(walk).radius(30.0).color(GREY);
    }

    draw_text(&draw);

    draw.to_frame(app, &frame).unwrap();
}
