use crate::modules::Rotation::*;
use crate::ScraenDim;

pub const PORT_NAME: &str = "/dev/ttyprintk";
// const PORT_NAME: &str = "/dev/ttyACM0";

pub const SCRAEN_SCALE: f32 = 10.0 * SCALE;

pub const LEFT_CAM: usize = 2;
pub const RIGHT_CAM: usize = 1;

pub const CAM_W: u32 = 320;
pub const CAM_H: u32 = 240;
pub const CAM_ORIENTATION: Orientation = Orientation::Portrait;

// pub const WIDTH: f32 = 240.0 * 2.0;
// pub const HEIGHT: f32 = 360.0 * 1.0;

pub const SCALE: f32 = 2.5;

pub const WINDOW_ASPECT_RATIO: f32 = 480.0 / 360.0;
pub const WIN_H: u32 = 800;
pub const WIN_W: u32 = (WIN_H as f32 * WINDOW_ASPECT_RATIO) as u32;

pub const SHOWDEBUG: bool = true;
pub const PRINT_AVALIBLE_PORTS: bool = false;

pub const PRINT_PORT_STATUS: bool = false;
pub const DOWNSCALE_FACTOR: f32 = 1.0;

pub const SCRAENS: [ScraenDim; 4] = [
    ScraenDim {
        rez: 4,
        xy: (366.0, -123.0),
        wh: (4.0, 4.0),
        rotation: Rotate90,
    },
    ScraenDim {
        rez: 16,
        xy: (-198.0, -212.0),
        wh: (16.0, 16.0),
        rotation: Rotate90,
    },
    ScraenDim {
        rez: 8,
        xy: (-238.0, 92.0),
        wh: (8.0, 8.0),
        rotation: Rotate270,
    },
    ScraenDim {
        rez: 12,
        xy: (153.0, 124.0),
        wh: (12.0, 12.0),
        rotation: Rotate270,
    },
];
pub enum Orientation {
    Portrait,
    Landscape,
}
