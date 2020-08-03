
use anyhow::Result;
use dht11::DHT11;
use lcd::Lcd;

use std::{thread, time::Duration};
fn main()-> Result<()> {
    let mut sensor = DHT11::new(21)?;
    let mut lcd = Lcd::new()?;
    lcd.init()?;

    loop {

        let result = sensor.read();

        match result {
            Ok(readings) => {
                println!("{}", readings);
                let msg = format!("Temp:      {:.1}C\nHumid:     {:.1}%", readings.temperature, readings.humidity);
                lcd.message(msg)?

            },

            Err(err) => eprintln!("{}",err)
        }

        
        thread::sleep(Duration::from_secs(3));
    }

}