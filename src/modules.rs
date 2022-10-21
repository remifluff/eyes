pub mod connection;
pub use connection::Connection;

pub mod scraen;
pub use scraen::Scraen;

pub mod webcam;
pub use webcam::Webcam;

pub mod walk;
pub use walk::Walk;

pub struct ScraenDim {
    pub rez: u32,
    pub xy: (f32, f32),
    pub wh: (f32, f32),
    pub rotation: Rotation,
}

pub struct Settings {
    min_radius: f32,
    max_radius: f32,
    circle_count: usize,
}

pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}
