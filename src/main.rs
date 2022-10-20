#![allow(dead_code)]
use nannou::prelude::*;

mod settings;
use settings::*;

mod modules;
use modules::*;

fn main() {
    nannou::app(model).update(update).run();
}
pub struct Model {
    scraens: Vec<Scraen>,

    port: Connection,

    face_cam_rect: Rect,
    street_cam_rect: Rect,
    target: Vec2,
    walk_x: Walk,
    walk_y: Walk,
    webcam: Webcam,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size((WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let win_rect = window.rect();

    let face_cam_rect =
        Rect::from_corners(win_rect.top_left(), win_rect.mid_bottom());
    let street_cam_rect =
        Rect::from_corners(win_rect.mid_top(), win_rect.bottom_right());

    let mut screen = Vec::new();
    for scraen_dim in SCRAENS {
        screen.push(Scraen::new(app, scraen_dim, face_cam_rect));
    }

    let mut port = Connection::new(PORT_NAME, PRINT_PORT_STATUS);
    port.open_port();

    if PRINT_AVALIBLE_PORTS {
        Connection::print_avaliable_ports();
    }

    Model {
        scraens: screen,
        // vision,
        port,
        face_cam_rect,
        street_cam_rect,
        target: vec2(0.0, 0.0),
        walk_x: Walk::new(43324),
        walk_y: Walk::new(621034),
        webcam: Webcam::new(app),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    let win = app.window_rect();

    // model.face_cam_rect
    model.webcam.update(app);

    // if let Some(t) = model.vision.get_target() {
    //     model.target = t;
    // };

    // model.target = app.mouse.position();
    let walk = vec2(model.walk_x.val(), model.walk_y.val())
        - model.face_cam_rect.xy();
    model.target = walk;

    model.walk_x.update();
    model.walk_y.update();

    model.port.write(vec![255]);
    for screen in &mut model.scraens {
        screen.draw_eye();
        screen.render_texture(&app);

        screen.update(&app, model.target, time.into());
        if let Some(buf) = screen.serial_packet() {
            model.port.write(buf);
        }
    }
    model.port.write(vec![254]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.webcam.draw(&draw);

    // model.webcam.draw_keypoints(&draw);
    // model.face_cam_rect;

    if SHOWDEBUG {
        for screen in &model.scraens {
            screen.draw_to_frame(&draw);
        }
    }

    let target2 = app.mouse.position();
    // let target = model.vision.biggest_face.xy();

    fn main() {
        println!("{:?}", chrono::offset::Local::now());
        println!("{:?}", chrono::offset::Utc::now());
    }

    let dt = chrono::offset::Local::now();
    dt.format("%Y-%m-%d %H:%M:%S");
    // font::collection_from_file( model/Futura.ttc)
    let walk = vec2(model.walk_x.val(), model.walk_y.val())
        - model.face_cam_rect.xy();

    if SHOWDEBUG {
        draw.ellipse().xy(walk).radius(30.0).color(GREY);
    }

    draw.text(
        format!("local time: {}", dt.format("%H:%M:%S:%f")).as_str(),
    )
    .color(WHITE)
    .font_size(24)
    .w_h(800.0, 10.0)
    .x_y(0.0, -370.0);
    draw.to_frame(app, &frame).unwrap();
}
