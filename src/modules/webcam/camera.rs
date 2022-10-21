use opencv::core::{flip, Vec3b};
use opencv::{core::*, prelude::*};

use opencv::{videoio, Result};
use opencv::{videoio::*, Error};

use super::Orientation;

pub struct Camera {
    cam: VideoCapture,
    img: Mat,
    rotated_img: Mat,

    pub data: Option<Vec<Vec<Vec3b>>>,
}
impl Camera {
    pub fn new(index: usize, width: u32, height: u32) -> Camera {
        let param: Vector<i32> = Vector::from_slice(&[
            CAP_PROP_FRAME_WIDTH,
            width as i32,
            CAP_PROP_FRAME_HEIGHT,
            height as i32,
        ]);

        let mut cam =
            VideoCapture::new_with_params(index as i32, 0, &param)
                .unwrap();
        VideoCapture::is_opened(&cam).expect("Open camera [FAILED]");

        cam.set(CAP_PROP_FPS, 30.0)
            .expect("Set camera FPS [FAILED]");

        Camera {
            cam,
            img: Mat::default(),
            rotated_img: Mat::default(),
            data: None,
        }
    }
    pub fn h(&self) -> u32 {
        self.cam
            .get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)
            .unwrap() as u32
    }
    pub fn w(&self) -> u32 {
        self.cam.get(opencv::videoio::CAP_PROP_FRAME_WIDTH).unwrap() as u32
    }

    pub fn get_frame(&mut self) -> Result<()> {
        let mut raw_frame = Mat::default();

        self.cam
            .read(&mut raw_frame)
            .expect("VideoCapture: read [FAILED]");

        if raw_frame.size()?.width > 0 {
            // flip the image horizontally
            // flip(&raw_frame, &mut flipped_frame, 1)
            //     .expect("flip [FAILED]");

            self.img = raw_frame;

            self.data = Some(self.img.to_vec_2d().unwrap());

            Ok(())
        } else {
            Err(Error {
                code: 0,
                message: "".to_owned(),
            })
        }
    }
    pub fn get_img(&mut self, orientation: &Orientation) -> &Mat {
        match orientation {
            Orientation::Portrait =>
            // rotate vertical
            {
                rotate(&self.img, &mut self.rotated_img, 2)
                    .expect("rotate [FAILED]");
                &self.rotated_img
            }
            Orientation::Landscape => &self.img,
        }
    }
}
