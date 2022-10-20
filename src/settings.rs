use crate::ScraenDim;

pub const PORT_NAME: &str = "/dev/ttyprintk";
// const PORT_NAME: &str = "/dev/ttyACM0";

pub const SCRAEN_SCALE: f32 = 10.0 * SCALE;
pub const CAMERA_WH: (u32, u32) = (320, 240);

pub const WIDTH: f32 = 240.0 * 2.0;
pub const HEIGHT: f32 = 360.0 * 1.0;

pub const SCALE: f32 = 2.5;

pub const SHOWDEBUG: bool = false;
pub const PRINT_AVALIBLE_PORTS: bool = false;

pub const PRINT_PORT_STATUS: bool = false;

pub const SCRAENS: [ScraenDim; 4] = [
    ScraenDim {
        rez: 4,
        xy: (466.0, -123.0),
        wh: (4.0, 4.0),
    },
    ScraenDim {
        rez: 16,
        xy: (102.0, -212.0),
        wh: (16.0, 16.0),
    },
    ScraenDim {
        rez: 8,
        xy: (38.0, 92.0),
        wh: (8.0, 8.0),
    },
    ScraenDim {
        rez: 12,
        xy: (453.0, 124.0),
        wh: (12.0, 12.0),
    },
];
