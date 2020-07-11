use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Działam na {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(23)?.into_output();
    pin.set_reset_on_drop(true);

    
    loop {
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }

    
    Ok(())
}
