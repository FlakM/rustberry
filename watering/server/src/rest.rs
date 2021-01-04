use crate::WateringDashboard;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantData {
    name: String,
    soil_humidity_readings: Vec<Reading>,
    watering_readings: Vec<Reading>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reading {
    x: String,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WateringChartData {
    from: DateTime<Local>,
    to: DateTime<Local>,
    sensor1: PlantData,
    sensor2: PlantData,
}


impl WateringChartData {
    pub fn from_dashboard(dashboard: &WateringDashboard) -> anyhow::Result<WateringChartData> {
        let mut sensor1_name: Option<String> = None;
        let mut sensor2_name = None;

        let mut sensor1_soil_humidity: Vec<Reading> = vec![];
        let mut sensor2_soil_humidity: Vec<Reading> = vec![];
        for reading in &dashboard.sensor_readings {
            match &reading.sensor[..] {
                "1" => {
                    if let None = sensor1_name {
                        sensor1_name = Some(reading.name.clone());
                    }
                    sensor1_soil_humidity.push(Reading {
                        x: reading.time.to_rfc3339(),
                        y: reading.value,
                    })
                }
                "2" => {
                    if let None = sensor2_name {
                        sensor2_name = Some(reading.name.clone());
                    }
                    sensor2_soil_humidity.push(Reading {
                        x: reading.time.to_rfc3339(),
                        y: reading.value,
                    })
                }
                _ => (),
            }
        }

        let mut sensor1_watering: Vec<Reading> = vec![];
        let mut sensor2_watering: Vec<Reading> = vec![];
        for reading in &dashboard.waterings {
            match &reading.sensor[..] {
                "1" => sensor1_watering.push(Reading {
                    x: reading.time.to_rfc3339(),
                    y: reading.duration_seconds as f32,
                }),
                "2" => sensor2_watering.push(Reading {
                    x: reading.time.to_rfc3339(),
                    y: reading.duration_seconds as f32,
                }),
                _ => (),
            }
        }

        Ok(WateringChartData {
            from: dashboard.from,
            to: dashboard.to,
            sensor1: PlantData {
                name: sensor1_name.unwrap(),
                soil_humidity_readings: sensor1_soil_humidity,
                watering_readings: sensor1_watering,
            },
            sensor2: PlantData {
                name: sensor2_name.unwrap(),
                soil_humidity_readings: sensor2_soil_humidity,
                watering_readings: sensor2_watering,
            },
        })
    }
}