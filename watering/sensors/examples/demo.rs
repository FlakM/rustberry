use std::error::Error;
use mcp3008::Mcp3008;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;


// 26 - kolendra
// 18 bazylia
const PUMP_GPIO: u8 = 18;

// 0 - bazylia
// 7 - kolendra
const SENSOR_CHANNEL: u8 = 0;


const SENSOR_POWER_GPIO: u8 = 16;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Działam na {}.", DeviceInfo::new()?.model());



    let mut pump_pin = Gpio::new()?.get(PUMP_GPIO)?.into_output();
    let mut sensor_power_pin = Gpio::new()?.get(SENSOR_POWER_GPIO)?.into_output();

    pump_pin.set_reset_on_drop(true);


    sensor_power_pin.set_high();

    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();


    let reading = mcp3008.read_adc(SENSOR_CHANNEL).unwrap();
    let humidity = calculate_percentage(reading);
    
    println!("Water level {}%", humidity );


    if humidity > 170.0 {
        println!("I shall not pump it no more!")
    } else {
        println!("In 2 seconds i will run a pump!");
        thread::sleep(Duration::from_millis(2000));
        
        println!("Pump it!");
        pump_pin.set_high();
        thread::sleep(Duration::from_millis(10000));
        println!("Stopped!");
        pump_pin.set_low();
    }

    sensor_power_pin.set_low();


    Ok(())
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculations() {
        assert_eq!(calculate_percentage(200), 100_f64);
        assert_eq!(calculate_percentage(710), 0_f64);
    }

}

fn calculate_percentage(sensor_value: u16) -> f64 {
    println!("converting reading {}", sensor_value);
    let min = 50; //wet
    let max = 710; //dry
    let sure = 100_f64;
    let percentage: f64 = (((sensor_value - min) * 100) / (max - min)).into();
    sure - percentage

}