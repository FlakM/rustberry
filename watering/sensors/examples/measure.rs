use std::error::Error;
use std::thread;
use mcp3008::Mcp3008;
use std::time::Duration;
use rppal::gpio::Gpio;

const SENSOR_CHANNEL: u8 = 0;
const SENSOR_POWER_GPIO: u8 = 16;

fn main() -> Result<(), Box<dyn Error>> {
    println!("hello world!");


    let mut sensor_power_pin = Gpio::new()?.get(SENSOR_POWER_GPIO)?.into_output();
    sensor_power_pin.set_high();
    thread::sleep(Duration::from_millis(500));
    
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();


    let reading = mcp3008.read_adc(SENSOR_CHANNEL).unwrap();

    println!("reading {}", reading);

    Ok(())
}