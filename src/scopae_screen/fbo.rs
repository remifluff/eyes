use crate::{Connection, Model, PORT};

use nannou::draw::properties::spatial::dimension;
use nannou::image::{self, DynamicImage, ImageBuffer, Pixel, Rgb, RgbImage};
use nannou::{draw, frame, prelude::*, wgpu::Device};
use serial2::SerialPort;

pub struct Fbo {
    pub texture: wgpu::Texture,
    draw: Draw,
    renderer: draw::Renderer,
    texture_size: [u32; 2],
    texture_capturer: wgpu::TextureCapturer,

    image: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pixel_count: u32,
}
unsafe impl Send for Fbo {}
impl Fbo {
    pub fn new(a: &App, dimensions: Point2) -> Fbo {
        let window = a.main_window();
        let device = window.device();

        let texture_capturer = wgpu::TextureCapturer::default();

        // // Create our custom texture.
        let sample_count = window.msaa_samples();
        // let texture = wgpu::TextureBuilder::new()
        //     .size(texture_size)
        //     // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
        //     // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
        //     .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        //     // Use nannou's default multisampling sample count.
        //     .sample_count(sample_count)
        //     // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
        //     .format(wgpu::TextureFormat::Rgba16Float)
        //     // Build it!
        //     .build(device);

        let texture_size = [
            u32::from_f32(dimensions.x).unwrap(),
            u32::from_f32(dimensions.y).unwrap(),
        ];

        let texture =
            wgpu::Texture::from_image(a, &DynamicImage::new_rgb8(texture_size[0], texture_size[1]));

        // Create our `Draw` instance and a renderer for it.
        let draw = nannou::Draw::new();
        let descriptor = texture.descriptor();
        let renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        Fbo {
            texture,
            texture_size,
            draw,
            renderer,
            texture_capturer,
            image: None,
            pixel_count: texture_size[0] * texture_size[1],
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

    pub fn print_image(&self) {
        if let Some(img) = &self.image {
            for pix in img.pixels() {
                // print!("{:?}", pix);
            }
        }
    }

    pub fn snapshot_texture(&self, a: &App, image_handler: fn(Vec<u8>)) {
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

        snapshot
            .read(move |result| {
                let image = result.expect("failed to map texture memory").to_owned();
                // let mut port = connection;

                let mut buf: Vec<u8> = Vec::new();
                for pix in image.pixels() {
                    let val = pix.to_luma().channels()[0];
                    buf.push(val);
                }
                image_handler(buf);
            })
            .unwrap();
    }
}

// fn snapshot_callback(r: Result<Rgba8AsyncMappedImageBuffer<'r>, BufferAsyncError>,){};
// self.texture_capturer.await_active_snapshots(device);
