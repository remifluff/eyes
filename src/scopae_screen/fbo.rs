use crate::{Connection, Model};

use nannou::draw::properties::spatial::dimension;
use nannou::image::{self, DynamicImage, ImageBuffer, Pixel, Rgb, RgbImage};
use nannou::{draw, frame, prelude::*, wgpu::Device};
use serial2::SerialPort;
use std::fmt::Error;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use anyhow::Result;

// use crate::{serial_Output, Model};

use ::image::{GenericImageView, Pixels};
use futures::future::ok;

pub struct Fbo {
    pub texture: wgpu::Texture,

    draw: Draw,
    renderer: draw::Renderer,
    resolution: (u32, u32),
    texture_capturer: wgpu::TextureCapturer,

    pub image_buffer: Arc<Mutex<DynamicImage>>,
    pixel_count: u32,
}
unsafe impl Send for Fbo {}
impl Fbo {
    pub fn new(a: &App, resolution: (u32, u32)) -> Fbo {
        let window = a.main_window();
        let device = window.device();

        let texture_capturer = wgpu::TextureCapturer::default();

        // // Create our custom texture.
        let sample_count = window.msaa_samples();

        // TextureResizer
        // let texture_size = [
        //     u32::from_f32(dimensions.x).unwrap(),
        //     u32::from_f32(dimensions.y).unwrap(),
        // ];
        let img = DynamicImage::new_rgb8(resolution.0, resolution.1);
        let image_capture = Arc::new(Mutex::new(img));

        let texture =
            wgpu::Texture::from_image(a, &DynamicImage::new_rgb8(resolution.0, resolution.1));

        // Create our `Draw` instance and a renderer for it.
        let draw = nannou::Draw::new();
        let descriptor = texture.descriptor();
        let renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        Fbo {
            texture,
            resolution,
            draw,
            renderer,
            texture_capturer,
            image_buffer: image_capture,
            pixel_count: resolution.0 * resolution.1,
            // texture_reshaper,
        }
    }

    pub fn draw(&self) -> &Draw {
        &self.draw
    }

    pub fn render(&self, app: &App) {
        let window = app.main_window();
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };

        let descriptor = self.texture.descriptor();
        let mut encoder = device.create_command_encoder(&ce_desc);
        let mut renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        // model.texture;

        renderer.render_to_texture(device, &mut encoder, &self.draw, &self.texture);
        window.queue().submit(Some(encoder.finish()));
    }

    pub fn snapshot_texture(&self, a: &App) {
        let window = a.main_window();
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };

        let mut encoder = device.create_command_encoder(&ce_desc);
        let snapshot = self
            .texture_capturer
            .capture(device, &mut encoder, &self.texture);

        window.queue().submit(Some(encoder.finish()));
        let buf = self.image_buffer.clone();
        snapshot
            .read(move |result| {
                if let Ok(buf) = &mut buf.lock() {
                    if let Ok(img) = result {
                        **buf = DynamicImage::ImageRgba8(img.to_owned())
                    }
                }
            })
            .unwrap();
    }
}
