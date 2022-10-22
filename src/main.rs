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

    left_cam: Rect,
    right_cam: Rect,

    win_rect: Rect,
    target: Vec2,
    walk_x: Walk,
    walk_y: Walk,
    webcam: Vec<Webcam>,
    ping_pong: bool,

    target_index: usize,
}

fn model(app: &App) -> Model {
    // let x = cycle(0, 1);
    let window_id = app
        .new_window()
        .size(WIN_W, WIN_H)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let win_rect = window.rect();
    let left_cam = win_rect.left_half();
    let right_cam = win_rect.left_half();

    let mut screen = Vec::new();
    for scraen_dim in SCRAENS {
        screen.push(Scraen::new(app, scraen_dim, win_rect));
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
        left_cam,
        right_cam,
        win_rect,
        target: vec2(0.0, 0.0),
        walk_x: Walk::new(43324),
        walk_y: Walk::new(621034),
        webcam: vec![
            Webcam::new(
                app,
                LEFT_CAM_INDEX,
                window.rect().left_half(),
                CAM_W,
                CAM_H,
                CAM_ORIENTATION,
            ),
            Webcam::new(
                app,
                RIGHT_CAM_INDEX,
                window.rect().right_half(),
                CAM_W,
                CAM_H,
                CAM_ORIENTATION,
            ),
        ],
        ping_pong: true,
        target_index: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    let win = app.window_rect();

    if SHOW_WEBCAMS {
        //randomly switch betweeen targets
        if random_range(0, 600) < 1 {
            if model.target_index == 0 {
                model.target_index = 1;
            } else {
                model.target_index = 0;
            }
        }

        //alternative update
        if model.ping_pong {
            model.webcam[0].update(app);
        } else {
            model.webcam[1].update(app);
        }
        model.ping_pong = !model.ping_pong;
    }

    model.walk_x.update();
    model.walk_y.update();

    model.target = match model.webcam[model.target_index].target {
        Some(target) => target.xy(),
        None => vec2(model.walk_x.val(), model.walk_y.val()) - model.win_rect.xy(),
    };

    // render screens and write to serial port
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
    // draw.background().color(BLACK);

    if SHOW_WEBCAMS {
        for webcam in &model.webcam {
            webcam.draw(&draw);
        }
    }

    let target2 = app.mouse.position();

    if SHOW_DEBUG {
        for screen in &model.scraens {
            screen.draw_to_frame(&draw);
        }
        draw.ellipse().xy(model.target).radius(30.0).color(GREY);
    }

    draw.to_frame(app, &frame).unwrap();
}

trait SidesExt {
    fn left_half(&self) -> Rect {
        Rect::from_w_h(0.0, 0.0)
    }
    fn right_half(&self) -> Rect {
        Rect::from_w_h(0.0, 0.0)
    }
}
impl SidesExt for Rect {
    fn left_half(&self) -> Rect {
        Rect::from_corners(self.top_left(), self.mid_bottom())
    }
    fn right_half(&self) -> Rect {
        Rect::from_corners(self.mid_top(), self.bottom_right())
    }
}

// fn main() {
//     println!("{:?}", chrono::offset::Local::now());
//     println!("{:?}", chrono::offset::Utc::now());
// }

// let dt = chrono::offset::Local::now();
// dt.format("%Y-%m-%d %H:%M:%S");
// font::collection_from_file( model/Futura.ttc)

// draw.text(
//     format!("local time: {}", dt.format("%H:%M:%S:%f")).as_str(),
// )
// .color(WHITE)
// .font_size(24)
// .w_h(800.0, 10.0)
// .x_y(0.0, -370.0);
