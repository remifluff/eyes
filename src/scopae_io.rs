pub mod serial_output;
use nannou::{draw, geom::cuboid::Face, prelude::Point2, App, Draw};
use nokhwa::Resolution;
pub use serial_output::SerialOutput;

pub mod webcam;
pub use webcam::Webcam;

pub mod face_detector;
pub use face_detector::FaceDetector;

const WEBCAM_WH: Resolution = Resolution {
    width_x: 320,
    height_y: 240,
};

pub struct ScopaeIo {
    face_cam: Webcam,
    stream_cam: Webcam,
    face_detector: FaceDetector,

    scale_factor: Point2,
    // wh: Point2,
}

impl ScopaeIo {
    pub fn new(app: &App) -> ScopaeIo {
        let mut face_cam = Webcam::new(app, 0, WEBCAM_WH);
        face_cam.initialize(app);

        let mut stream_cam = Webcam::new(app, 1, WEBCAM_WH);
        stream_cam.initialize(app);

        ScopaeIo {
            face_cam,
            stream_cam,
            face_detector: FaceDetector::new(),
            scale_factor: Point2::new(0.0, 0.0),
        }
    }

    // vision.initialize();

    // vision.update_camera(app, win);

    pub fn update(&mut self, app: &App) {
        let win = app.window_rect();

        self.face_cam.capture_camera_frame(app);

        self.stream_cam.capture_camera_frame(app);

        // self.face_detector.update_faces(&self.face_cam.image);

        // self.scale_factor = screen.wh() / self.wh;
        // self.scale_factor = Point2::from([self.scale_factor.max_element(); 2]);
    }
    pub fn draw(&self, app: &App, draw: &Draw) {
        let win = app.window_rect();

        // offset: Point2, scale_factor: Point2

        self.face_cam.draw_camera(draw, win);
    }
}

// draw.texture(&self.texture?)
// .wh(self.wh * scale_factor * vec2(-1.0, 1.0))
// .xy(vec2(0.0, 0.0));

// self.stream_cam.update_camera(app, win);

// model.vision.draw_camera(&draw, offset);
// model.vision.draw_face(&draw, win, offset);
