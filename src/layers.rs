    \\ Put in your model() function, so you can save the texture in your model
    let window_size = window.rect(); // Main window size
    let device = window.device();
    let texture_for_layer = wgpu::TextureBuilder::new()
        .size([window_size.w() as u32, window_size.h() as u32])
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        .sample_count(1)
        .format(wgpu::TextureFormat::Rgba16Float)
        .build(device);
     ...

    \\ In your update() function you draw whatever you have to draw in your layer
    \\ Then you create a render pass to specify to WGPU backend to draw
    \\ on a texture instead of final surface
    let device = window.device();
    let ce_desc = wgpu::CommandEncoderDescriptor {
        label: Some("texture renderer"),
    };
    let mut encoder = device.create_command_encoder(&ce_desc);
    model
        .renderer
        .render_to_texture(device, &mut encoder, &draw, &model.texture_for_layer);
    \\ This will draw all draw operations on the selected texture
    \\ all other (following) draw operations will be executed on default frame buffer if not specified differently (as here)
    window.queue().submit(Some(encoder.finish()));

   \\ Then finally in your view() function you blit the texture on the
   \\ surface
    let draw = app.draw();
    draw.texture(&model.texture_for_layer);