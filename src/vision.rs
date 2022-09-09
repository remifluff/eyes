use crate::{Connection, Fbo, Model, PORT};
use nannou::image::{DynamicImage, GrayImage};
use nannou::prelude::*;
use nokhwa::{Camera, CameraFormat, FrameFormat};
use rustface::{Detector, FaceInfo, ImageData};
use wgpu::{TextueSnapshot, Texture};

pub struct Vision {
    image: DynamicImage,
    faces: Vec<FaceInfo>,
    camera: Camera,
    texture: Texture,
    detector: Box<dyn Detector>,

    screen_position: Point2,
    wh: Point2,
}

impl Vision {
    pub fn new(app: &App, model_path: &str, wh: Point2) -> Vision {
        let mut detector = rustface::create_detector(model_path).unwrap();

        detector.set_min_face_size(40);
        detector.set_score_thresh(2.0);
        detector.set_pyramid_scale_factor(0.8);
        detector.set_slide_window_step(4, 4);

        let mut camera = Camera::new(
            0,
            Some(CameraFormat::new_from(
                u32::from_f32(wh.x).unwrap(),
                u32::from_f32(wh.y).unwrap(),
                FrameFormat::MJPEG,
                30,
            )), // format
        )
        .unwrap();

        camera.open_stream().unwrap();

        let image = DynamicImage::ImageRgb8(camera.frame().unwrap()).thumbnail(100, 80);
        let texture = Texture::from_image::<&App>(app, &image);

        Vision {
            image,
            faces: Vec::new(),
            texture,
            camera,

            detector,
            screen_position: Point2::new(0.0, 0.0),
            wh,
        }
    }
    pub fn initialize(&self) {}

    pub fn update(&mut self, app: &App) {
        self.image = DynamicImage::ImageRgb8(self.camera.frame().unwrap()).thumbnail(200, 140);
        self.texture = Texture::from_image::<&App>(app, &self.image);

        self.faces = {
            let detector: &mut dyn Detector = &mut *self.detector;
            let gray = self.image.clone().to_luma8();
            let (width, height) = gray.dimensions();
            let mut image = ImageData::new(&gray, width, height);
            let faces = detector.detect(&mut image);
            faces
        };
    }

    pub fn draw_camera(&self, app: &App) {
        app.draw().texture(&self.texture).w_h(640.0, 480.0);
    }

    pub fn draw_face(&self, app: &App) {
        for face in &self.faces {
            let bbox = face.bbox();
            app.draw()
                .rect()
                .w_h(
                    f32::from_u32(bbox.width()).unwrap(),
                    f32::from_u32(bbox.height()).unwrap(),
                )
                .x_y(
                    f32::from_i32(bbox.x()).unwrap(),
                    f32::from_i32(bbox.y()).unwrap(),
                );
        }
    }

    // let mut rgb = model.image.unwrap().to_rgb8();
}
