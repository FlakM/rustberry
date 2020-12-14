use serde::{Deserialize, Serialize};

// todo implement Responder  according to https://actix.rs/docs/handlers/
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadingDb {
    pub time: chrono::NaiveDateTime,
    pub sensor: String,
    pub metric: String,
    pub value: f32,
}

#[derive(Serialize, Deserialize, Debug)]
//insert into water_history (time, sensor,duration_seconds) values ( now(), $1, $2 )
pub struct WateringTimeRecordings {
    pub time: chrono::NaiveDateTime,
    pub sensor: String,
    pub duration_seconds: i32,
}

#[derive(Deserialize)]
pub struct Info {
    // todo rename fields and make them longs?
    pub start: Option<String>,
    pub finish: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WateringDashboard {
    pub from: chrono::NaiveDateTime,
    pub to: chrono::NaiveDateTime,
    pub sensor_readings: Vec<ReadingDb>,
    pub waterings: Vec<WateringTimeRecordings>,
}