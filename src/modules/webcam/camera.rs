use opencv::core::{flip, Vec3b};
use opencv::{core::*, prelude::*};

use opencv::{videoio, Result};
use opencv::{videoio::*, Error};

pub struct Camera {
    pub cam: VideoCapture,
    pub img: Mat,

    pub data: Option<Vec<Vec<Vec3b>>>,
}
impl Camera {
    pub fn new(index: usize) -> Camera {
        let mut cam =
            videoio::VideoCapture::new(index as i32, videoio::CAP_ANY)
                .unwrap();
        videoio::VideoCapture::is_opened(&cam)
            .expect("Open camera [FAILED]");

        cam.set(CAP_PROP_FPS, 30.0)
            .expect("Set camera FPS [FAILED]");

        Camera {
            cam,
            img: Mat::default(),
            data: None,
        }
    }
    pub fn w(&self) -> u32 {
        self.cam.get(opencv::videoio::CAP_PROP_FRAME_WIDTH).unwrap() as u32
    }
    pub fn h(&self) -> u32 {
        self.cam
            .get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)
            .unwrap() as u32
    }

    pub fn get_frame(&mut self) -> Result<()> {
        let mut frame = Mat::default();

        self.cam
            .read(&mut frame)
            .expect("VideoCapture: read [FAILED]");

        if frame.size()?.width > 0 {
            // flip the image horizontally

            flip(&frame, &mut self.img, 1).expect("flip [FAILED]");
            // resize the image as a square, size is

            self.data = Some(frame.to_vec_2d().unwrap());

            Ok(())
        } else {
            Err(Error {
                code: 0,
                message: "".to_owned(),
            })
        }
    }
}
