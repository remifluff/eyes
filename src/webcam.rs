type Result<T> = std::result::Result<T, WebcamError>;

#[derive(Debug, Clone)]
struct WebcamError;

use nannou::prelude::*;
use nannou::{App, Draw};
use opencv::core::{flip, Vec3b};
use opencv::{core::*, imgproc::*, prelude::*};

use opencv::videoio::*;
use opencv::{highgui::*, prelude::*, videoio};

mod movenet;
use movenet::*;

mod camera;
use camera::*;

pub struct Webcam {
    movenet: Movenet,
    cam: Camera,
}
impl Webcam {
    pub fn new(app: &App) -> Webcam {
        Webcam {
            movenet: Movenet::new(),
            cam: Camera::new(app),
        }
    }

    pub fn update(&mut self, app: &App) {
        self.movenet.update(self.cam.update().unwrap());

        self.cam.update();
        self.cam.update_texture(app);

        // imshow("MoveNet", &self.cam.img).expect("imshow [ERROR]");
        // keypress check
        let key = wait_key(1).unwrap();
        // if key > 0 && key != 255 {
        //     break;
        // }
    }

    pub fn draw_camera(&self, draw: &Draw) {
        // let t = self.cam.cam_to_screen;
        draw.texture(&self.cam.texture);

        // .xy(get_t_xy(cam.cam_rect, t))
        // .wh(get_t_wh(cam.cam_rect, t));
        // if let Some(_) = cam.backend {
        // } else {
        //     draw.rect()
        //         .xy(get_t_xy(cam.cam_rect, t))
        //         .wh(get_t_wh(cam.cam_rect, t))
        //         .color(WHITE);
        // }
    }

    pub fn draw_keypoints(&self, draw: &Draw) {
        let img: &Mat = &self.cam.img;
        let keypoints = self.movenet.data();
        // keypoints: [1, 17, 3]
        let base: f32;
        let pad_x: f32;
        let pad_y: f32;
        if img.rows() > img.cols() {
            base = img.rows() as f32;
            pad_x = (img.rows() - img.cols()) as f32 / 2.0;
            pad_y = 0.0;
        } else {
            base = img.cols() as f32;
            pad_x = 0.0;
            pad_y = (img.cols() - img.rows()) as f32 / 2.0;
        }

        for index in 0..17 {
            let y_ratio = keypoints[index * 3];
            let x_ratio = keypoints[index * 3 + 1];
            let confidence = keypoints[index * 3 + 2];
            if confidence > 0.25 {
                let xy = vec2(
                    (x_ratio * base) - pad_x,
                    (y_ratio * base) - pad_y,
                );

                draw.ellipse().xy(xy).radius(5.0).color(WHITE);

                // circle(
                //     img,

                //     0,
                //     Scalar::new(0.0, 255.0, 0.0, 0.0),
                //     5,
                //     LINE_AA,
                //     0,
                // )
                // .expect("Draw circle [FAILED]");
            }
        }

        // if let Ok(faces) = self.faces.lock() {
        //     for face in faces.iter() {
        //         let t = cam.cam_to_screen;

        // }
    }

    /*
    pub fn draw_face(&self, draw: &Draw, screen: Rect) {
        let cam = &self.webcams[0];

        if let Ok(faces) = self.faces.lock() {
            for face in faces.iter() {
                let t = cam.cam_to_screen;

                draw.ellipse()
                    .xy(get_t_xy(*face, t))
                    .radius(get_t_wh(*face, t).x)
                    .color(WHITE);
            }
        }
    }

     */
}

pub fn draw_keypoints(img: &mut Mat, keypoints: &[f32], threshold: f32) {
    // keypoints: [1, 17, 3]
    let base: f32;
    let pad_x: i32;
    let pad_y: i32;
    if img.rows() > img.cols() {
        base = img.rows() as f32;
        pad_x = (img.rows() - img.cols()) / 2;
        pad_y = 0;
    } else {
        base = img.cols() as f32;
        pad_x = 0;
        pad_y = (img.cols() - img.rows()) / 2;
    }

    for index in 0..17 {
        let y_ratio = keypoints[index * 3];
        let x_ratio = keypoints[index * 3 + 1];
        let confidence = keypoints[index * 3 + 2];
        if confidence > threshold {
            circle(
                img,
                Point {
                    x: (x_ratio * base) as i32 - pad_x,
                    y: (y_ratio * base) as i32 - pad_y,
                },
                0,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                5,
                LINE_AA,
                0,
            )
            .expect("Draw circle [FAILED]");
        }
    }
}
