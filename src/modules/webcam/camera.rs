use std::thread;
use std::time::Duration;

use opencv::core::{flip, Vec3b};
use opencv::{core::*, prelude::*};

use opencv::{videoio, Result};
use opencv::{videoio::*, Error};

use super::Orientation;

use CameraStatus::*;
use FrameStatus::*;

pub struct Camera {
    cam: Option<VideoCapture>,
    img: Mat,
    rotated_img: Mat,
    pub frame_status: FrameStatus,

    camera_params: (usize, u32, u32),

    pub data: Option<Vec<Vec<Vec3b>>>,
}
impl Camera {
    pub fn new(index: usize, width: u32, height: u32) -> Camera {
        let camera_params = (index, width, height);
        let cam = Self::open_camera(camera_params);
        Camera {
            camera_params,
            cam,
            img: Mat::default(),
            rotated_img: Mat::default(),
            data: None,
            frame_status: NothingRead,
        }
    }

    pub fn open_camera(camera_params: (usize, u32, u32)) -> Option<VideoCapture> {
        let (index, width, height) = camera_params;

        let param: Vector<i32> = Vector::from_slice(&[
            CAP_PROP_FRAME_WIDTH,
            width as i32,
            CAP_PROP_FRAME_HEIGHT,
            height as i32,
        ]);

        let mut cam = VideoCapture::new_with_params(index as i32, 0, &param).unwrap();

        // need both of these aswell as the parameters for some reason
        cam.set(CAP_PROP_FPS, 30.0)
            .expect("Set camera FPS [FAILED]");

        cam.set(CAP_PROP_FRAME_WIDTH, width as f64)
            .expect("Set camera FPS [FAILED]");

        match VideoCapture::is_opened(&cam) {
            Ok(result) => match result {
                true => Some(cam),
                false => None,
            },
            Err(_) => None,
        }
    }

    pub fn get_frame(&mut self, time: f32) {
        let mut raw_frame = Mat::default();

        if let Some(cam) = &mut self.cam {
            match cam.read(&mut raw_frame).unwrap() {
                true => {
                    self.img = raw_frame;
                    self.data = Some(self.img.to_vec_2d().unwrap());
                    self.frame_status = FrameRead(time);
                }
                false => {
                    self.frame_status = FrameReadFailed(time);
                    self.cam = None;
                }
            }
        } else {
            self.cam = Self::open_camera(self.camera_params)
        }

        // match self.cam {
        //     Some(cam) => match cam.read(&mut raw_frame) {
        //         Ok(result) => match result {
        //             true => {}
        //             false => FrameReadFailed(time),
        //         },
        //         Err(_) => NothingRead,
        //     },
        //     None => (),
        // }
    }

    // pub fn h(&self) -> Option<u32> {
    //     Some(
    //         self.cam?
    //             .get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)
    //             .unwrap() as u32,
    //     )
    // }
    // pub fn w(&self) -> Option<u32> {
    //     Some(
    //         self.cam?
    //             .get(opencv::videoio::CAP_PROP_FRAME_WIDTH)
    //             .unwrap() as u32,
    //     )
    // }
    pub fn get_img(&mut self, orientation: &Orientation) -> &Mat {
        match orientation {
            Orientation::Portrait =>
            // rotate vertical
            {
                rotate(&self.img, &mut self.rotated_img, 2).expect("rotate [FAILED]");
                &self.rotated_img
            }
            Orientation::Landscape => &self.img,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CameraStatus {
    Connected,
    OpenFailed,
    InvalidSettings,
}
#[derive(Debug, Clone, Copy)]
pub enum FrameStatus {
    NothingRead,
    FrameRead(f32),
    FrameReadFailed(f32),
}
