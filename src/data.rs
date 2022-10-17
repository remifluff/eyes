use chrono;

use nannou::{prelude::WHITE, Draw};

pub fn draw_text(draw: &Draw) {
    let dt = chrono::offset::Local::now();
    dt.format("%Y-%m-%d %H:%M:%S");
    // font::collection_from_file( model/Futura.ttc)

    draw.text(
        format!("local time: {}", dt.format("%H:%M:%S:%f")).as_str(),
    )
    .color(WHITE)
    .font_size(24)
    .w_h(800.0, 10.0)
    .x_y(0.0, -370.0);
}
