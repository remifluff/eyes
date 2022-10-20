use nannou::prelude::*;
use nannou::{App, Draw};
use opencv::core::{flip, Vec3b};
use opencv::objdetect::CascadeClassifier;
use opencv::{
    core::*,
    imgproc::{self, *},
    prelude::*,
    types, Result,
};

use opencv::videoio::*;
use opencv::{highgui::*, objdetect, prelude::*, videoio};

mod camera;
use camera::*;

pub struct Webcam {
    cam: Camera,
    face: CascadeClassifier,
}
impl Webcam {
    pub fn new(app: &App) -> Webcam {
        let xml = find_file(
            "haarcascades/haarcascade_frontalface_alt.xml",
            true,
            false,
        )
        .unwrap();

        Webcam {
            cam: Camera::new(app),
            face: objdetect::CascadeClassifier::new(&xml).unwrap(),
        }
    }

    pub fn do_faces(&mut self) -> Result<()> {
        let reduced = self.cam.face_detect_frame().unwrap();
        let mut faces = types::VectorOfRect::new();

        self.face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            Size {
                width: 30,
                height: 30,
            },
            Size {
                width: 0,
                height: 0,
            },
        )?;
        println!("faces: {}", faces.len());
        for face in faces {
            println!("face {:?}", face);
            let scaled_face = opencv::core::Rect {
                x: face.x * 4,
                y: face.y * 4,
                width: face.width * 4,
                height: face.height * 4,
            };
            // imgproc::rectangle(
            //     &mut frame,
            //     scaled_face,
            //     core::Scalar::new(0f64, -1f64, -1f64, -1f64),
            //     1,
            //     8,
            //     0,
            // )?;
        }
        Ok(())
    }

    pub fn update(&mut self, app: &App) {
        self.cam.update();
        self.cam.update_texture(app);
    }

    pub fn draw_camera(&self, draw: &Draw) {
        draw.texture(&self.cam.texture);
    }
}
