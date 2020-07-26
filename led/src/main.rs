use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use std::sync::{Arc, Mutex};

//https://docs.golemparts.com/rppal/0.11.2/rppal/gpio/struct.OutputPin.html#note

fn main() -> Result<(), Box<dyn Error>> {
    println!("Działam na {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(23)?.into_output();
    let closed = Arc::new(Mutex::new(false));

    let closed_handler = closed.clone();
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        println!("set to closed");
        *closed_handler.lock().unwrap() = true;
    })?;


    while !*closed.lock().unwrap() {
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }

    println!("setting to low");
    pin.set_low();
    Ok(())
}

mod tests {

    #[test]
    fn test() {
        {
            let a = 1;
            // some computations
        } // a get dropped here
    }
}
