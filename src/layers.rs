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





    // fn draw_into_texture(a: &App, m: &Model, d: &Draw) {
//     d.reset();
//     d.background().color(BLACK);
//     let [w, h] = (m.screen.dim.to_array());
//     let r = geom::Rect::from_w_h(w as f32, h as f32);

//     let elapsed_frames = a.main_window().elapsed_frames();
//     let t = elapsed_frames as f32 / 60.0;

//     let n_points = 10;
//     let weight = 8.0;
//     let hz = 6.0;
//     let vertices = (0..n_points)
//         .map(|i| {
//             let x = map_range(i, 0, n_points - 1, r.left(), r.right());
//             let fract = i as f32 / n_points as f32;
//             let amp = (t + fract * hz * TAU).sin();
//             let y = map_range(amp, -1.0, 1.0, r.bottom() * 0.75, r.top() * 0.75);
//             pt2(x, y)
//         })
//         .enumerate()
//         .map(|(i, p)| {
//             let fract = i as f32 / n_points as f32;
//             let r = (t + fract) % 1.0;
//             let g = (t + 1.0 - fract) % 1.0;
//             let b = (t + 0.5 + fract) % 1.0;
//             let rgba = srgba(r, g, b, 1.0);
//             (p, rgba)
//         });

//     d.polyline()
//         .weight(weight)
//         .join_round()
//         .points_colored(vertices);

//     // Draw frame number and size in bottom left.
//     let string = format!("Frame {} - {:?}", elapsed_frames, [w, h]);
//     let text = text(&string)
//         .font_size(48)
//         .left_justify()
//         .align_bottom()
//         .build(r.pad(r.h() * 0.05));

//     d.path().fill().color(WHITE).events(text.path_events());
// }
