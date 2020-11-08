
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt;

/// I'm using following sensor
/// https://allegro.pl/oferta/czujnik-wilgotnosci-gleby-m335-odporny-na-korozje-9367467102
/// It reads values from 0-3V which are translated by MCP3008 to range 0-1023
/// They must be calibrated by noting maximal dry/wet setting
#[derive(Serialize, Deserialize, Debug)]
pub struct SoilSensorParams {
    pub water_reading: u16,
    pub air_reading: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plant {
    pub id: String,
    pub name: String,
    pub analog_channel: u8,
    pub pump_gpio: u8,
    pub water_for_seconds: u64,
    pub sensor_params: SoilSensorParams,
    pub requires_watering_level: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterLevelSensor {
    analog_channel: u8,
    water_reading: u16,
    just_a_tip_reading: u16,
    air_reading: u16
}

#[derive(Serialize, Deserialize)]
pub struct Setup {
    pub water_level_sensor: WaterLevelSensor,
    pub plants: Vec<Plant>
}

pub struct SoilHumidityReading{pub humidity: f64}


pub struct PlantsState {
    pub plants: Vec<(Plant, SoilHumidityReading)>,
    pub water_container_reading: WaterLevelReading
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


pub enum WaterLevelReading {
    NoWaterContact, NotComplete, CompleteUnderWater
}

pub fn calculate_percentage(sensor_value: u16, sensor_calibration: &SoilSensorParams) -> f64 {
    let min = sensor_calibration.water_reading;
    let max = sensor_calibration.air_reading;


    if sensor_value < sensor_calibration.water_reading {
        100_f64
    } else if sensor_value > sensor_calibration.air_reading {
        0_f64
    } else {
        let calibrated: f64 = (((sensor_value - min) * 100) / (max - min)).into();
        100_f64 - calibrated
    }
}