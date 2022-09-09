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

extern crate rustface;

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
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let options = match Options::parse(std::env::args()) {
        Ok(options) => options,
        Err(message) => {
            println!("Failed to parse program arguments: {}", message);
            std::process::exit(1)
        }
    };

    let mut detector = match rustface::create_detector(options.model_path()) {
        Ok(detector) => detector,
        Err(error) => {
            println!("Failed to create detector: {}", error.to_string());
            std::process::exit(1)
        }
    };

    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    // let image: DynamicImage = match image::open(options.image_path()) {
    //     Ok(image) => image,
    //     Err(message) => {
    //         println!("Failed to read image: {}", message);
    //         std::process::exit(1)
    //     }
    // };

    let mut rgb = model.image.unwrap().to_rgb8();
    let faces = detect_faces(&mut *detector, &model.image.unwrap().to_luma8());

    for face in faces {
        let bbox = face.bbox();
        let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());

        draw_hollow_rect_mut(&mut rgb, rect, Rgb([255, 0, 0]));
    }

    match rgb.save(OUTPUT_FILE) {
        Ok(_) => println!("Saved result to {}", OUTPUT_FILE),
        Err(message) => println!("Failed to save result to a file. Reason: {}", message),
    }
    // println!("{}, {}", frame.width(), frame.height());

    let t = app.time;

    model.eye.set_center(app.mouse.position());
    model.eye.update_openess(t.blink_ease(1.0));
    if model.write_timer.check(t) {}

    model.image = Some(ImageRgb8(model.camera.frame().unwrap()));

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
    println!(
        "Found {} faces in {} ms",
        faces.len(),
        get_millis(now.elapsed())
    );
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

// fn draw_into_texture(a: &App, m: &Model, d: &Draw) {
//     d.reset();
//     d.background().color(BLACK);
//     let [w, h] = (m.screen.dim.to_array());
//     let r = geom::Rect::from_w_h(w as f32, h as f32);

//     let elapsed_frames = a.main_window().elapsed_frames();
//     let t = elapsed_frames as f32 / 60.0;

//     let n_points = 10;
//     let weight = 8.0;
//     let hz = 6.0;
//     let vertices = (0..n_points)
//         .map(|i| {
//             let x = map_range(i, 0, n_points - 1, r.left(), r.right());
//             let fract = i as f32 / n_points as f32;
//             let amp = (t + fract * hz * TAU).sin();
//             let y = map_range(amp, -1.0, 1.0, r.bottom() * 0.75, r.top() * 0.75);
//             pt2(x, y)
//         })
//         .enumerate()
//         .map(|(i, p)| {
//             let fract = i as f32 / n_points as f32;
//             let r = (t + fract) % 1.0;
//             let g = (t + 1.0 - fract) % 1.0;
//             let b = (t + 0.5 + fract) % 1.0;
//             let rgba = srgba(r, g, b, 1.0);
//             (p, rgba)
//         });

//     d.polyline()
//         .weight(weight)
//         .join_round()
//         .points_colored(vertices);

//     // Draw frame number and size in bottom left.
//     let string = format!("Frame {} - {:?}", elapsed_frames, [w, h]);
//     let text = text(&string)
//         .font_size(48)
//         .left_justify()
//         .align_bottom()
//         .build(r.pad(r.h() * 0.05));

//     d.path().fill().color(WHITE).events(text.path_events());
// }

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

fn get_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000u64 + u64::from(duration.subsec_nanos() / 1_000_000)
}

struct Options {
    image_path: String,
    model_path: String,
}

impl Options {
    fn parse(args: Args) -> Result<Self, String> {
        let args: Vec<String> = args.into_iter().collect();
        if args.len() != 3 {
            return Err(format!("Usage: {} <model-path> <image-path>", args[0]));
        }

        let model_path = args[1].clone();
        let image_path = args[2].clone();

        Ok(Options {
            image_path,
            model_path,
        })
    }

    fn image_path(&self) -> &str {
        &self.image_path[..]
    }

    fn model_path(&self) -> &str {
        &self.model_path[..]
    }
}
