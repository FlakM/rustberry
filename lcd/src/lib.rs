use anyhow::{anyhow, Result};

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::{thread, time};

use std::convert::TryFrom;

pub struct Lcd {
    /// cztery piny do przesyłu danych (bit 4-7)
    pub data: [OutputPin; 4],

    /// pin wyboru rejestru
    pub rs: OutputPin,

    /// pin enable
    pub en: OutputPin,
}

const ROW_OFFSET: [u8; 2] = [0x00, 0x40];

// commands:
const LCD_SETDDRAMADDR: u8 = 0x80;

impl Lcd {
    pub fn new() -> Result<Lcd> {
        let rs = Gpio::new()?.get(22)?.into_output();

        let enable = Gpio::new()?.get(5)?.into_output();

        let d4 = Gpio::new()?.get(26)?.into_output();
        let d5 = Gpio::new()?.get(19)?.into_output();
        let d6 = Gpio::new()?.get(13)?.into_output();
        let d7 = Gpio::new()?.get(6)?.into_output();

        Ok(Lcd {
            data: [d4, d5, d6, d7],
            rs: rs,
            en: enable,
        })
    }

    pub fn init(&mut self) -> Result<()> {
        // checkout Mode selection in  https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller
        // this will set controller to 4-bit mode no matter the entering state
        self.write(0x33, false)?;
        self.write(0x32, false)?;

        thread::sleep(time::Duration::new(0, 4 * 1000)); // wait?

        // Turn display on and display control
        self.write(0x08 | 0x04, false)?;
        thread::sleep(time::Duration::new(0, 4 * 1000)); // wait?

        // Set Entry left mode
        self.write(0x06, false)?;
        thread::sleep(time::Duration::new(0, 4 * 1000)); // wait?

        self.clear()?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.write(0x01, false)?; // command to clear display LCD_CLEARDISPLAY
        thread::sleep(time::Duration::new(0, 1570000)); // 1,57 ms sleep, clearing the display takes a long time
        Ok(())
    }

    fn process_msg(text: &str) -> Result<Vec<u8>> {
        if text.lines().count() > 2 {
            return Err(anyhow!(
                "Invalid line count. We have only 2 lines on screen"
            ));
        }

        for (i, line) in text.lines().map(|l| l.replace("\n", "")).enumerate() {
            if line.chars().count() > 16 {
                return Err(anyhow!(format!(
                    "Line number {} has more then 16 allowed characters",
                    i
                )));
            }
        }

        let mut result = vec![];

        for char in text.chars() {
            result.push(u8::try_from(char as u32)?);
        }

        Ok(result)
    }

    pub fn message(&mut self, text: String) -> Result<()> {
        self.clear()?;
        let mut line = 0;

        // let characters = text.chars();
        let characters = Lcd::process_msg(&text)?;

        for char in characters {
            if char == '\n' as u8 {
                line += 1;
                self.set_cursor(0, line)?
            } else {
                // todo perform some check of casting
                self.write(char as u8, true)?
            }
        }
        Ok(())
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) -> Result<()> {
        let mut actual_row = row;
        // Clamp row to the last row of the display.
        if row > 2 {
            // max row
            actual_row = 1
        }
        self.write(
            LCD_SETDDRAMADDR | (col + ROW_OFFSET[actual_row as usize]),
            false,
        )?;
        Ok(())
    }

    fn pulse_enable(&mut self) {
        let write_pin = |pin: &mut OutputPin, enabled: bool| {
            if enabled {
                pin.set_high()
            } else {
                pin.set_low()
            }
        };

        // Breathing time
        thread::sleep(time::Duration::new(0, 450));

        // enable pulse must be > 450ns
        write_pin(&mut self.en, false);
        thread::sleep(time::Duration::new(0, 1000));

        // enable pulse must be > 450ns
        write_pin(&mut self.en, true);
        thread::sleep(time::Duration::new(0, 450));

        // commands need 37us to settle
        write_pin(&mut self.en, false);
        thread::sleep(time::Duration::new(0, 37 * 1000));
    }

    pub fn write(&mut self, value: u8, char_mode: bool) -> Result<()> {
        let write_pin = |pin: &mut OutputPin, enabled: bool| {
            if enabled {
                pin.set_high()
            } else {
                pin.set_low()
            }
        };

        write_pin(&mut self.en, false);
        write_pin(&mut self.rs, char_mode);

        write_pin(&mut self.data[0], value & 0b0001_0000u8 > 0);
        write_pin(&mut self.data[1], value & 0b0010_0000u8 > 0);
        write_pin(&mut self.data[2], value & 0b0100_0000u8 > 0);
        write_pin(&mut self.data[3], value & 0b1000_0000u8 > 0);

        self.pulse_enable();

        write_pin(&mut self.data[0], value & 0b0000_0001u8 > 0);
        write_pin(&mut self.data[1], value & 0b0000_0010u8 > 0);
        write_pin(&mut self.data[2], value & 0b0000_0100u8 > 0);
        write_pin(&mut self.data[3], value & 0b0000_1000u8 > 0);

        self.pulse_enable();

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_our_validation() {
        assert!(Lcd::process_msg("hello \n world").is_ok());
        assert!(Lcd::process_msg("hello \n world\n").is_ok());
        assert!(Lcd::process_msg("hello \n world\n third line\n").is_err());
        assert!(Lcd::process_msg("hello this is a long sentence\n world").is_err());
        assert!(Lcd::process_msg("ą").is_err());
    }

    #[test]
    fn print() {
        println!("0x33 {} {:008b}",0x33 as u8, 0x33);
        println!("0x32 {} {:008b}",0x33 as u8, 0x32);
    }

}
