pub mod serial_output;
use nannou::{draw, geom::cuboid::Face, prelude::Point2, App};
pub use serial_output::SerialOutput;

pub mod webcam;
pub use webcam::Webcam;

pub mod face_detector;
pub use face_detector::FaceDetector;

const WEBCAM_WH: (u32, u32) = (320, 240);

pub struct ScopaeIo {
    face_cam: Webcam,
    stream_cam: Webcam,
    face_detector: FaceDetector,

    scale_factor: Point2,
    // wh: Point2,
}

impl ScopaeIo {
    pub fn new(app: &App) -> ScopaeIo {
        ScopaeIo {
            face_cam: Webcam::new(app, WEBCAM_WH),
            stream_cam: Webcam::new(app, WEBCAM_WH),
            face_detector: FaceDetector::new(),
            scale_factor: Point2::new(0.0, 0.0),
        }
    }
    pub fn update(&self, app: &App) {
        let win = app.window_rect();

        self.face_cam.update_camera(app, win);

        self.stream_cam.update_camera(app, win);
        // unsafe {
        //     CAMERA_READY = false;
        // }

        self.face_detector.update_faces(&self.face_cam.image);

        // self.scale_factor = screen.wh() / self.wh;
        // self.scale_factor = Point2::from([self.scale_factor.max_element(); 2]);
    }
    pub fn draw(&self, app: &App) {
        let win = app.window_rect();

        self.face_cam.update_camera(app, win);

        self.stream_cam.update_camera(app, win);

        model.vision.draw_camera(&draw, offset);
        model.vision.draw_face(&draw, win, offset);
    }
}
