use crate::{CommLibResult, InvalidLed, NotAscii, Port, SendError, TooLong};
use crate::CommLibError::ReadError;
use crate::manager::Update::LED;
use crate::LED_GREEN;
use crate::LED_BLUE;
use crate::LED_RED;

const COMMAND_BUTTON_PRESSED: u8 = 0x01;
const COMMAND_BUTTON_RELEASED: u8 = 0x02;
const COMMAND_SET_LED: u8 = 0x03;
const COMMAND_SET_TEXT: u8 = 0x04;

const COMMAND_LED_BLUE: u8 = 0;
const COMMAND_LED_RED: u8 = 1;
const COMMAND_LED_GREEN: u8 = 2;
const COMMAND_LED_OFF: u8 = 0;
const COMMAND_LED_ON: u8 = 1;

/// Used to communicate with hardware device
/// All calls are blocking
pub struct DeviceManager {
    port: Port,
    buttons: [bool; 4],
}

impl DeviceManager {
    pub fn new(port: Port) -> Self {
        DeviceManager { port, buttons: [false, false, false, false] }
    }
}

impl DeviceManager {
    /// Send update to device
    pub fn send(&mut self, update: Update) -> CommLibResult<()> {
        update.validate()?;
        match update {
            Update::LED(_, _) => {
                self.port.write(&[
                    COMMAND_SET_LED,
                    update.get_command_led(),
                    update.get_command_led_state()]
                ).map_err(|err| SendError(err.to_string()))?;
            }
            Update::Text(str) => {
                let mut bytes = str.chars().map(|c| c as u8).collect::<Vec<u8>>();
                bytes.insert(0, COMMAND_SET_TEXT);
                self.port.write(&bytes)
                    .map_err(|err| SendError(err.to_string()))?;
            }
        }
        Ok(())
    }

    /// Update manager device state
    pub fn recv(&mut self) -> CommLibResult<()> {
        match self.port.bytes_to_read() {
            Ok(num) => {
                if num > 1 {
                    let mut data = [0, 0];
                    self.port.read_exact(&mut data).map_err(|err| ReadError(err.to_string()))?;
                    match data[0] {
                        COMMAND_BUTTON_PRESSED => self.buttons[data[1] as usize] = true,
                        COMMAND_BUTTON_RELEASED => self.buttons[data[1] as usize] = false,
                        _ => println!("Unknown command: {:?}", data)
                    }
                }
            }
            Err(err) => return Err(ReadError(err.description))
        }
        Ok(())
    }

    /// Returns last known button state
    pub fn get_button_state(&self) -> [bool; 4] {
        self.buttons
    }
}

pub enum Update {
    LED(usize, bool),
    Text(String),
}

impl Update {
    /// Check that values inside the udpate are valid
    pub fn validate(&self) -> CommLibResult<()> {
        match self {
            Update::LED(id, _) => if id > &2 { return Err(InvalidLed(*id)); }
            Update::Text(text) => {
                let mut invalids = vec![];
                if text.chars().count() > 84 {
                    return Err(TooLong);
                }
                for (i, chr) in text.chars().enumerate() {
                    if !chr.is_ascii_graphic() && chr as u8 != 32 {
                        invalids.push(i);
                    }
                }
                if !invalids.is_empty() {
                    return Err(NotAscii(invalids));
                }
            }
        }
        Ok(())
    }

    fn get_command_led(&self) -> u8 {
        if let LED(id, _) = self {
            return match *id {
                LED_GREEN => COMMAND_LED_GREEN,
                LED_BLUE => COMMAND_LED_BLUE,
                LED_RED => COMMAND_LED_RED,
                _ => panic!("Impossible")
            };
        }
        panic!("Impossible")
    }

    fn get_command_led_state(&self) -> u8 {
        if let LED(_, state) = self {
            return if *state {
                COMMAND_LED_ON
            } else {
                COMMAND_LED_OFF
            };
        }
        panic!("Impossible")
    }
}
