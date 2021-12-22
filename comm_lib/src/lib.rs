pub mod manager;

use serialport::SerialPort;
use thiserror::Error;
use crate::CommLibError::{InvalidLed, NoDeviceFound, NotAscii, NotSupported, SendError, TooLong};

pub type Port = Box<dyn SerialPort>;
pub type CommLibResult<T> = Result<T, CommLibError>;

#[derive(Error, Debug)]
pub enum CommLibError {
    #[error("Some characters in the string were not ASCII: {0:?}")]
    NotAscii(Vec<usize>),
    #[error("{0}")]
    NotSupported(String),
    #[error("No device found")]
    NoDeviceFound,
    #[error("Error reading from device: {0}")]
    ReadError(String),
    #[error("Error sending to device: {0}")]
    SendError(String),
    #[error("Invalid LED, must be 0, 1 or 2, was {0}")]
    InvalidLed(usize),
    #[error("Text is too long, max 84 chars")]
    TooLong,
}

pub const LED_GREEN: usize = 0;
pub const LED_BLUE: usize = 1;
pub const LED_RED: usize = 2;

pub fn get_potential_devices() -> CommLibResult<Vec<String>> {
    match serialport::available_ports() {
        Ok(ports) => {
            Ok(ports.iter()
                .filter(|p| p.port_name.contains("usbmodem"))
                .map(|p| p.port_name.clone())
                .collect())
        }
        Err(err) => Err(NotSupported(err.description))
    }
}

pub fn get_best_match_device() -> CommLibResult<Port> {
    let mut list = get_potential_devices()?;
    if list.is_empty() {
        Err(NoDeviceFound)
    } else {
        let port = serialport::new(list.remove(0), 9600)
            .open()
            .map_err(|err| NotSupported(err.description))?;
        Ok(port)
    }
}


