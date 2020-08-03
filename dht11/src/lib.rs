use anyhow::{anyhow, Result};

use rppal::gpio::Gpio;
use rppal::gpio::{IoPin, Level, Mode};
use std::fmt;
use std::time::{Duration, Instant};
use std::{thread, time};

pub struct DHT11 {
    pin: IoPin,
}

#[derive(Debug)]
pub struct Readings {
    pub temperature: f64,
    pub humidity: f64,
}

impl fmt::Display for Readings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Readings({:.2}%, {:.2}C)",
            self.humidity, self.temperature
        )
    }
}

impl DHT11 {
    pub fn new(gpio_num: u8) -> Result<DHT11> {
        Ok(DHT11 {
            pin: Gpio::new()?.get(gpio_num)?.into_io(Mode::Output),
        })
    }

    /// initialize
    fn init_sequence(&mut self) -> () {
        // step 2 of documentation
        self.pin.set_mode(Mode::Output);
        self.pin.set_low();
        delay_ms(18);
        self.pin.set_high();
        delay_us(50)
    }

    pub fn read(&mut self) -> Result<Readings> {
        self.init_sequence();

        let mut bytes = [0u8; 5];

        {
            // step 3 of documentation
            self.pin.set_mode(Mode::Input);
            wait_level(&mut self.pin, Level::Low)?;
            wait_level(&mut self.pin, Level::High)?;
            wait_level(&mut self.pin, Level::Low)?;

            // step 4 of documentation
            for b in bytes.iter_mut() {
                for _ in 0..8 {
                    *b <<= 1;
                    wait_level(&mut self.pin, Level::High)?;
                    let dur = wait_level(&mut self.pin, Level::Low)?;
                    if dur > 26 {
                        *b |= 1;
                    }
                }
            }
        }

        let sum: u16 = bytes.iter().take(4).map(|b| *b as u16).sum();
        if bytes[4] as u16 == sum & 0x00FF {
            Ok(Readings {
                temperature: bytes[2] as f64 + (bytes[3] as f64 / 10.0),
                humidity: bytes[0] as f64 + (bytes[1] as f64 / 10.0),
            })
        } else {
            Err(anyhow!("Check sum!"))
        }
    }
}

fn wait_level(pin: &mut IoPin, level: Level) -> Result<u8> {
    for i in 0u8..255 {
        if pin.read() == level {
            return Ok(i);
        }
        delay_us(1);
    }
    Err(anyhow!("Timeout!"))
}

pub fn delay_ms(ms: u64) {
    let millis = time::Duration::from_millis(ms);
    thread::sleep(millis);
}

pub fn delay_us(us: u32) {
    let target = Instant::now() + Duration::new(0, us * 1000);
    while Instant::now() < target {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn understand_bitwise_operations() {
        let mut start: u8 = 1;

        for _ in 0..8 {
            start <<= 1;
            println!("{:08b}", start);
        }
    }
}
