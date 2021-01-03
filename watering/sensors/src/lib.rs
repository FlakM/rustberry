use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::fmt;
use chrono::NaiveTime;



/// I'm using following sensor
/// https://allegro.pl/oferta/czujnik-wilgotnosci-gleby-m335-odporny-na-korozje-9367467102
/// It reads values from 0-3V which are translated by MCP3008 to range 0-1023
/// They must be calibrated by noting maximal dry/wet setting
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoilSensorParams {
    pub water_reading: u16,
    pub air_reading: u16,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WateringParams {
    /// how long a plant should be watered
    pub water_for_seconds: i32,
    /// time of the day from which plant shpild be watered
    pub water_start_time: NaiveTime, 
    /// end time og the day from which plant should be watered
    pub water_end_time: NaiveTime, 
    /// percentage of soil level that requires watering
    pub requires_watering_level: f32,
    /// number of pin that starts pump
    pub pump_gpio: u8,
}

impl WateringParams {
    pub fn should_be_watered(&self) -> bool {
        let current_time = chrono::offset::Local::now().time();
        current_time < self.water_end_time && current_time > self.water_start_time
    }
}


pub fn settings_from_json(path_to_config: &str) -> anyhow::Result<Setup> {
    let content = std::fs::read_to_string(path_to_config)?;
    let setup = Setup::from_str(&content)?;
    Ok(setup)
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parsing_default_json() -> anyhow::Result<()> {
        let setup: Setup = settings_from_json("./config.json").unwrap();
        let first: &Plant = setup.plants.get(0).unwrap();

        assert_eq!(first.watering_params.pump_gpio, 18);
        let from_time=first.watering_params.water_start_time;
        assert_eq!(from_time, NaiveTime::from_hms(8, 0, 0));


        let last: &Plant = setup.plants.get(1).unwrap();
        assert_eq!(last.watering_params.pump_gpio, 26);

        Ok(())
    }


    #[test]
    fn checking_water_level() {
        let setup: Setup = settings_from_json("./config.json").unwrap();
        let mut plant: Plant = setup.plants.get(0).unwrap().clone();

        let now = chrono::offset::Local::now().time();

        let start = now - chrono::Duration::minutes(10);
        let end = now + chrono::Duration::minutes(10);


        plant.watering_params.water_start_time = start;
        plant.watering_params.water_end_time = end;

        assert_eq!(plant.watering_params.should_be_watered(), true);

        let start = now - chrono::Duration::minutes(20);
        let end = now - chrono::Duration::minutes(10);


        plant.watering_params.water_start_time = start;
        plant.watering_params.water_end_time = end;

        assert_eq!(plant.watering_params.should_be_watered(), false)
    }


}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plant {
    pub id: String,
    pub name: String,
    pub analog_channel: u8,
    pub sensor_params: SoilSensorParams,
    pub watering_params: WateringParams
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaterLevelSensor {
    pub analog_channel: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Setup {
    pub water_level_sensor: WaterLevelSensor,
    pub sensor_power_pin: u8,
    pub plants: Vec<Plant>
}

pub struct SoilHumidityReading{pub humidity: f32}


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
#[derive(Debug)]

pub enum WaterLevelReading {
    NoWaterContact, CompleteUnderWater
}

pub fn calculate_water_level(level: u16) -> WaterLevelReading {
    if level < 50 {
        WaterLevelReading::NoWaterContact
    } else {
        WaterLevelReading::CompleteUnderWater
    }
}


pub fn calculate_percentage(sensor_value: u16, sensor_calibration: &SoilSensorParams) -> f32 {
    let min = sensor_calibration.water_reading;
    let max = sensor_calibration.air_reading;

    if sensor_value < sensor_calibration.water_reading {
        100_f32
    } else if sensor_value > sensor_calibration.air_reading {
        0_f32
    } else {
        let calibrated: f32 = (((sensor_value - min) * 100) / (max - min)).into();
        100_f32 - calibrated
    }
}