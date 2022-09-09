use nannou::prelude::*;
// #[derive(Debug, PartialEq)]

pub struct Connection<'a> {
    port_name: &'a str,
    port: Option<SerialPort>,
    connected: bool,
    print_activity: bool,
}
use serial2::SerialPort;

use crate::PORT;

impl Connection<'_> {
    pub const fn new(port_name: &str, print_activity: bool) -> Connection {
        Connection {
            port: None,
            connected: false,
            print_activity,
            port_name,
        }
    }

    pub fn print_avaliable_ports() {
        for path in SerialPort::available_ports().unwrap() {
            println!("{:?}", path);
        }
    }

    pub fn write(&mut self, mut vec: Vec<u8>) {
        unsafe {
            PORT.open_port();
        }
        if let Some(port) = &self.port {
            port.write(&vec);
        } else {
        }
    }

    pub fn open_port(&mut self) {
        match SerialPort::open(&self.port_name, 115200) {
            Ok(p) => {
                if !self.connected {
                    self.port = Some(p);

                    self.connected = true;
                    if self.print_activity {
                        println!("port connected");
                    }
                }
            }
            Err(_Err) => {
                self.connected = false;

                if self.print_activity {
                    println!("couldn't open port: {}", { _Err });
                }
            }
        }
    }
}
