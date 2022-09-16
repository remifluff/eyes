use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

// use nokhwa::{query, Camera, CameraFormat, FrameFormat, ThreadedCamera};
// use rustface::FaceInfo;

use image::{GenericImageView, ImageBuffer, Rgb};
use nannou::draw::primitive::rect;
use nannou::image::{DynamicImage, GrayImage};
use nannou::lyon::geom::euclid::point2;
use nannou::lyon::math::Point;
use nannou::prelude::*;

use anyhow::{anyhow, Result};
use nokhwa::{Camera, CameraFormat, FrameFormat, Resolution, ThreadedCamera};
use rustface::{Detector, FaceInfo, ImageData};
use std::sync::mpsc::{self, channel, Receiver, Sender};
use wgpu::{TextueSnapshot, Texture};

const MODEL_PATH: &str = "model/seeta_fd_frontal_v1.0.bin";

enum FrameStatus {
    Unloaded,
    NewFrameProcessing,
    FrameReady,
    OldFrame,
}

pub struct Webcam {
    camera: Result<ThreadedCamera>,
    pub image: Option<DynamicImage>,
    texture: Option<Texture>,
    wh: Point2,

    resolution: Resolution,

    frame_status: Arc<Mutex<FrameStatus>>,
}

impl Webcam {
    pub fn new(app: &App, webcam_number: usize, resolution: Resolution) -> Webcam {
        // let image = image::open("model/faces.jpg").unwrap();

        let mut wh = Point2::new(resolution.height() as f32, resolution.width() as f32);

        let format = CameraFormat::new(resolution, FrameFormat::MJPEG, 30);

        let camera = ThreadedCamera::new(webcam_number, Some(format)).map_err(|e| anyhow!("sop"));

        Webcam {
            image: None,
            texture: None,
            camera: camera,
            wh,
            resolution,
            frame_status: Arc::new(Mutex::new(FrameStatus::Unloaded)),
        }
    }
    pub fn initialize(&mut self, app: &App) -> Option<()> {
        if let Ok(cam) = &mut self.camera {
            cam.open_stream(every_frame_callback);
        }

        Some(())
    }

    pub fn capture_camera_frame(&mut self, app: &App) -> Option<()> {
        if let Ok(cam) = &mut self.camera {
            if let Ok(img) = &mut cam.poll_frame() {
                let imgg = DynamicImage::ImageRgb8(img.clone());

                self.texture = Some(Texture::from_image::<&App>(app, &imgg));

                self.image = Some(imgg);
            };
        }
        Some(())
    }

    pub fn draw_camera(&self, draw: &Draw, rect: Rect) {
        if let Some(tex) = &self.texture {
            draw.texture(tex).wh(rect.wh()).xy(rect.xy());
        }
    }
}
fn every_frame_callback(image: nannou::image::ImageBuffer<Rgb<u8>, Vec<u8>>) {}
