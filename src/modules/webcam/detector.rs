use opencv::{
    core::{find_file, Rect_, Size, Vector},
    imgproc,
    objdetect::{self, CascadeClassifier},
    prelude::*,
    types, Result,
};

pub struct Detector {
    face: CascadeClassifier,
    pub faces: Vector<Rect_<i32>>,
}

impl Detector {
    pub fn new() -> Detector {
        let xml = find_file(
            "haarcascades/haarcascade_frontalface_alt.xml",
            true,
            false,
        )
        .unwrap();

        Detector {
            face: objdetect::CascadeClassifier::new(&xml).unwrap(),
            faces: types::VectorOfRect::new(),
        }
    }
    pub fn update_faces(&mut self, img: &Mat) -> Result<()> {
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
            0.25f64,
            0.25f64,
            imgproc::INTER_LINEAR,
        )?;

        self.faces = types::VectorOfRect::new();

        self.face.detect_multi_scale(
            &reduced,
            &mut self.faces,
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

        Ok(())
    }
}
