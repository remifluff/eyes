use opencv::{
    core::{find_file, Rect_, Size, Vector},
    imgproc,
    objdetect::{self, CascadeClassifier},
    prelude::*,
    types, Result,
};

use crate::settings::Orientation;

pub struct Detector {
    face: CascadeClassifier,
    pub faces: Vec<(f32, f32, f32, f32)>,
    downscale_factor: f32,
    pub biggest_rect: Option<((f32, f32, f32, f32))>,
}

impl Detector {
    pub fn new(downscale_factor: f32) -> Detector {
        let xml = find_file(
            "haarcascades/haarcascade_frontalface_alt.xml",
            true,
            false,
        )
        .unwrap();

        Detector {
            face: objdetect::CascadeClassifier::new(&xml).unwrap(),
            faces: Vec::new(),
            downscale_factor,
            biggest_rect: None,
        }
    }
    pub fn update_faces(
        &mut self,
        img: &Mat,
        orientation: &Orientation,
    ) -> Result<()> {
        let mut gray = Mat::default();
        imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut reduced = Mat::default();
        imgproc::resize(
            &gray,
            &mut reduced,
            Size {
                width: 0,
                height: 0,
            },
            self.downscale_factor as f64,
            self.downscale_factor as f64,
            imgproc::INTER_LINEAR,
        )?;

        let mut faces = types::VectorOfRect::new();

        self.face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            Size {
                width: 10,
                height: 10,
            },
            Size {
                width: 0,
                height: 0,
            },
        )?;

        self.faces = faces
            .iter()
            .map(|face| {
                let w = face.width as f32 / self.downscale_factor;
                let h = face.height as f32 / self.downscale_factor;
                let x = face.x as f32 / self.downscale_factor;
                let y = face.y as f32 / self.downscale_factor;

                let (offset_x, offset_y): (f32, f32) = match orientation {
                    Orientation::Portrait => (w, 0.0),
                    Orientation::Landscape => (w / 2.0, h / 2.0),
                };
                (x + offset_x, y + offset_y, w, h)
            })
            .collect();

        // pub fn get_target(&mut self) -> Option<Point2> {
        //     if let Ok(faces) = self.faces.lock() {
        let biggest_rect = self
            .faces
            .iter()
            .min_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

        self.biggest_rect = if let Some(x) = biggest_rect {
            Some(*x)
        } else {
            None
        };
        //     }
        //     let t = self.webcams[0].cam_to_screen;

        //     // .xy(get_t_xy(*face, t))
        //     // .radius(get_t_wh(*face, t).x)

        //     Some(t.transform_point2(self.biggest_face.xy()))
        // }

        Ok(())
    }
}
