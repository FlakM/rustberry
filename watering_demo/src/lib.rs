
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt;

/// I'm using following sensor
/// https://allegro.pl/oferta/czujnik-wilgotnosci-gleby-m335-odporny-na-korozje-9367467102
/// It reads values from 0-3V which are translated by MCP3008 to range 0-1023
/// They must be calibrated by noting maximal dry/wet setting
#[derive(Serialize, Deserialize, Debug)]
pub struct SoilSensorParams {
    min: u16,
    max: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plant {
    name: String,
    analog_channel: u8,
    pump_gpio: u8,
    sensor_params: SoilSensorParams,
}

#[derive(Serialize, Deserialize)]
pub struct Setup {
    pub plants: Vec<Plant>
}

impl fmt::Debug for Setup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let plants_str = format!("{:?}",&self.plants);

        f.debug_struct("Setup")
         .field("plants", &plants_str)
         .finish()
    }
}

impl Setup {
    pub fn from_str(s: &str) -> Result<Setup> {
        serde_json::from_str(s)
    }
}