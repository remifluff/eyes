use crate::{Connection, Fbo, Model, PORT};
use nannou::image::{DynamicImage, GrayImage};
use nannou::prelude::*;
use nokhwa::{Camera, CameraFormat, FrameFormat};
use rustface::{Detector, FaceInfo, ImageData};
use wgpu::{TextueSnapshot, Texture};

pub struct Vision {
    image: Option<DynamicImage>,
    faces: Vec<FaceInfo>,
    camera: Camera,
    texture: Option<Texture>,
    detector: Box<dyn Detector>,

    screen_position: Point2,
    wh: Point2,
}

impl Vision {
    pub fn new(model_path: &str, wh: Point2) -> Vision {
        let mut detector = rustface::create_detector(model_path).unwrap();

        detector.set_min_face_size(20);
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

        Vision {
            image: None,
            faces: Vec::new(),
            texture: None,
            camera,

            detector,
            screen_position: Point2::new(0.0, 0.0),
            wh,
        }
    }
    pub fn initialize(&self) {}

    pub fn update(&mut self, app: &App) {
        self.image = Some(DynamicImage::ImageRgb8(self.camera.frame().unwrap()));
        self.texture = Some(Texture::from_image::<&App>(
            app,
            &self.image.as_ref().unwrap(),
        ));

        self.faces = detect_faces(
            &mut *self.detector,
            &self.image.as_ref().unwrap().to_luma8(),
        );

        fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
            let (width, height) = gray.dimensions();
            let mut image = ImageData::new(gray, width, height);
            let faces = detector.detect(&mut image);
            faces
        }
    }

    pub fn draw_camera(&self, app: &App) {
        app.draw()
            .texture(&self.texture.as_ref().unwrap())
            .w_h(600.0, 600.0);
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
