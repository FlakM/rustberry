
use anyhow::Result;

use lcd::Lcd;
use std::{thread, time};

fn main() -> Result<()> {

    let mut pins = Lcd::new()?;

    pins.init()?;
    pins.message(String::from("witaj swiecie \n   rust :)"))?;
    thread::sleep(time::Duration::new(2, 0));

    pins.message(String::from("witaj swiecie \n   rust ;)"))?;
    thread::sleep(time::Duration::new(2, 0));

    pins.clear()?;
    thread::sleep(time::Duration::new(2, 0));

    Ok(())
}