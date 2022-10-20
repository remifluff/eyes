pub mod connection;
pub use connection::Connection;

pub mod scraen;
pub use scraen::Scraen;

pub mod webcam;
pub use webcam::Webcam;

pub mod timer;
pub use timer::Timer;

pub mod walk;
pub use walk::Walk;

pub struct ScraenDim {
    pub rez: u32,
    pub xy: (f32, f32),
    pub wh: (f32, f32),
}

pub struct Settings {
    min_radius: f32,
    max_radius: f32,
    circle_count: usize,
}
