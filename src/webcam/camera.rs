use image::DynamicImage;
use nannou::prelude::Vec2;
use nannou::prelude::*;
use nannou::{app, App};

use nannou_wgpu::Texture;
use opencv::core::{flip, Vec3b};
use opencv::{core::*, imgproc::*, prelude::*};

use opencv::videoio::*;
use opencv::{highgui::*, prelude::*, videoio};

use super::Result;
use super::WebcamError;

pub struct Camera {
    pub cam: VideoCapture,
    pub img: Mat,
    pub video_size: Vec2,
    pub texture: Texture,
    data: Option<Vec<Vec<Vec3b>>>,
}
impl Camera {
    pub fn new(app: &App) -> Camera {
        // Resize input

        // open camera
        let mut cam =
            videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(); // 0 is the default camera

        let width =
            cam.get(opencv::videoio::CAP_PROP_FRAME_WIDTH).unwrap();
        let height =
            cam.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT).unwrap();
        let video_size = Vec2::new(width as f32, height as f32);

        let empty = &DynamicImage::new_rgb8(
            video_size.x as u32,
            video_size.y as u32,
        );

        videoio::VideoCapture::is_opened(&cam)
            .expect("Open camera [FAILED]");

        cam.set(CAP_PROP_FPS, 30.0)
            .expect("Set camera FPS [FAILED]");

        Camera {
            cam,
            img: Mat::default(),
            video_size,
            texture: Texture::from_image::<&App>(app, empty),
            data: None,
        }
    }
    pub fn update(&mut self) -> Result<Vec<u8>> {
        let mut frame = Mat::default();

        self.cam
            .read(&mut frame)
            .expect("VideoCapture: read [FAILED]");

        if frame.size().unwrap().width > 0 {
            // flip the image horizontally

            flip(&frame, &mut self.img, 1).expect("flip [FAILED]");
            // resize the image as a square, size is
            let resized_img = resize_with_padding(&self.img, [192, 192]);

            // turn Mat into Vec<u8>
            let vec_2d: Vec<Vec<opencv::core::Vec3b>> =
                resized_img.to_vec_2d().unwrap();

            let vec_1d: Vec<u8> = vec_2d
                .iter()
                .flat_map(|v| v.iter().flat_map(|w| w.as_slice()))
                .cloned()
                .collect();

            self.data = Some(frame.to_vec_2d().unwrap());

            Ok(vec_1d)
        } else {
            Err(WebcamError)
        }
    }

    pub fn update_texture(&mut self, app: &App) {
        if let Some(data) = &mut self.data {
            // if self.frame_data.is_empty() || self.frame_data[0].is_empty() {
            //     return;
            // }

            let width = self.video_size.x as u32;
            let height = self.video_size.y as u32;

            let image =
                image::ImageBuffer::from_fn(width, height, |x, y| {
                    let pixel = data[y as usize][(width - x - 1) as usize];
                    image::Rgb([
                        pixel[2] as u8,
                        pixel[1] as u8,
                        pixel[0] as u8,
                    ])
                });

            let img = DynamicImage::ImageRgb8(image.clone());
            // .rotate270()
            self.texture = Texture::from_image::<&App>(app, &img);

            // let flat_samples = image.as_flat_samples();
            // let byte_vec = floats_as_byte_vec(flat_samples.as_slice());

            // self.texture.upload_data(device, encoder, &byte_vec[..]);
        }
    }

    pub fn img(&self) -> &Mat {
        &self.img
    }
}

pub fn resize_with_padding(img: &Mat, new_shape: [i32; 2]) -> Mat {
    let img_shape = [img.cols(), img.rows()];
    let width: i32;
    let height: i32;
    if img_shape[0] as f64 / img_shape[1] as f64
        > new_shape[0] as f64 / new_shape[1] as f64
    {
        width = new_shape[0];
        height = (new_shape[0] as f64 / img_shape[0] as f64
            * img_shape[1] as f64) as i32;
    } else {
        width = (new_shape[1] as f64 / img_shape[1] as f64
            * img_shape[0] as f64) as i32;
        height = new_shape[1];
    }

    let mut resized = Mat::default();
    resize(
        img,
        &mut resized,
        Size { width, height },
        0.0,
        0.0,
        INTER_LINEAR,
    )
    .expect("resize_with_padding: resize [FAILED]");

    let delta_w = new_shape[0] - width;
    let delta_h = new_shape[1] - height;
    let (top, bottom) = (delta_h / 2, delta_h - delta_h / 2);
    let (left, right) = (delta_w / 2, delta_w - delta_w / 2);

    let mut rslt = Mat::default();
    copy_make_border(
        &resized,
        &mut rslt,
        top,
        bottom,
        left,
        right,
        BORDER_CONSTANT,
        Scalar::new(0.0, 0.0, 0.0, 0.0),
    )
    .expect("resize_with_padding: copy_make_border [FAILED]");
    rslt
}
pub fn float_as_bytes(data: &f32) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

pub fn floats_as_byte_vec(data: &[f32]) -> Vec<u8> {
    let mut bytes = vec![];
    data.iter().for_each(|f| bytes.extend(float_as_bytes(f)));
    bytes
}
