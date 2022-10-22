use crate::modules::Rotation::*;
use crate::ScraenDim;

// pub const PORT_NAME: &str = "/dev/ttyprintk";
pub const PORT_NAME: &str = "/dev/ttyACM0";

pub const LEFT_CAM_INDEX: usize = 0;
pub const RIGHT_CAM_INDEX: usize = 2;

pub const CAM_W: u32 = 320;
pub const CAM_H: u32 = 240;
pub const CAM_ORIENTATION: Orientation = Orientation::Portrait;
pub const DOWNSCALE_FACTOR: f32 = 0.5;

pub const SCALE: f32 = 2.5;
pub const SCRAEN_SCALE: f32 = 10.0 * SCALE;

pub const WINDOW_ASPECT_RATIO: f32 = 480.0 / 320.0;
pub const WIN_H: u32 = 600;
pub const WIN_W: u32 = (WIN_H as f32 * WINDOW_ASPECT_RATIO) as u32;

pub const SHOW_DEBUG: bool = true;
pub const SHOW_WEBCAMS: bool = true;

pub const PRINT_AVALIBLE_PORTS: bool = false;
pub const PRINT_PORT_STATUS: bool = false;

pub const BLINK_CHANCE_PER_FRAME: f32 = 1.0 / 100.0;
pub const EYE_ACCELERATION: f32 = 0.1;

pub const BLINK_SECS_TO_CLOSE: f64 = 0.1;
pub const BLINK_SECS_STAY_CLOSE: f64 = 0.2;
pub const BLINK_SECS_TO_OPEN: f64 = 0.2;

pub const WEBCAM_SETTINGS: [WebcamSettings; 2] = [
    WebcamSettings {
        index: LEFT_CAM_INDEX,
        camera_width: CAM_W,
        camera_height: CAM_H,
        orientation: CAM_ORIENTATION,
    },
    WebcamSettings {
        index: RIGHT_CAM_INDEX,
        camera_width: CAM_W,
        camera_height: CAM_H,
        orientation: CAM_ORIENTATION,
    },
];

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

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Portrait,
    Landscape,
}
#[derive(Debug, Clone, Copy)]
pub struct WebcamSettings {
    pub index: usize,
    pub camera_width: u32,
    pub camera_height: u32,
    pub orientation: Orientation,
}
