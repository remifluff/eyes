use anyhow::{anyhow, Error, Result};
use nannou::prelude::*;

// #[derive(Debug, PartialEq)]

pub struct SerialOutput {
    port_name: String,
    port: Result<SerialPort>,
    connected: bool,
    print_activity: bool,
}
use serial2::SerialPort;

// use crate::scopae_screen::fbo::Fbo;

impl SerialOutput {
    pub const fn new(port_name: &str, print_activity: bool) -> SerialOutput {
        SerialOutput {
            port_name: todo!(),
            port: todo!(),
            connected: todo!(),
            print_activity,
            // port: Err(anyhow!("Port not connected")),
            // connected: false,
            // print_activity,
            // port_name: port_name.to_owned(),
        }
    }

    pub fn print_avaliable_ports() {
        for path in SerialPort::available_ports().unwrap() {
            println!("{:?}", path);
        }
    }

    // pub fn write_FBO(&self, mut buffer: Vec<u8>, fbo: Fbo) -> Result<()> {
    //     let mut buf: Vec<u8> = Vec::new();

    //     // let a = *fbo.image_buffer.try_lock().map_err(|e| anyhow!("somethings weirddd"))?;

    //     // for pix in a.pixels() {
    //     //     let val = pix.to_luma().channels()[0];
    //     //     buf.push(val);
    //     // }

    //     let mut send_buffer: Vec<u8> = Vec::new();

    //     for (pos, e) in buffer.iter().enumerate() {
    //         let col_index = pos % 12;
    //         let row_index = pos / 12;

    //         match pos {
    //             0 => send_buffer.push(255),
    //             _ if col_index == 0 => {
    //                 send_buffer.push(0);
    //                 send_buffer.push(clamp(*e, 0u8, 200u8));
    //             }
    //             _ if (row_index % 2) == 0 => {
    //                 send_buffer.push(clamp(*e, 0u8, 20u8));
    //             }
    //             _ => send_buffer.push(clamp(*e, 0u8, 200u8)),
    //         };
    //     }
    //     // ((pos / 12)) 0
    //     send_buffer.push(254);

    //     // self.port?.write(&send_buffer);

    //     Ok(())
    // }

    // pub fn write(&mut self, mut vec: Vec<u8>) {
    //     self.open_port();
    //     if let Some(port) = &self.port {
    //         port.write(&vec);
    //     } else {
    //     }

    // }

    pub fn open_port(&mut self) {
        // self.port = SerialPort::open(&self.port_name, 115200).map_err(|e| anyhow!("posrt baddddd"));

        // self.connected = match self.port {
        //     Ok(_) if !self.connected => {
        //         println!("port connected");
        //         true
        //     }
        //     Ok(_) => true,

        //     Err(_Err) => {
        //         println!("couldn't open port: {}", { _Err });
        //         false
        //     }
        // }
    }
}
