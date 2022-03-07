// extern crate openssl;

use models::*;
use rppal::gpio::Gpio;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use mcp3008::Mcp3008;

mod mail;

async fn read_plants_state(setup: Setup) -> anyhow::Result<PlantsState> {
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0")?;
    let mut sensor_pin = Gpio::new()?.get(setup.sensor_power_pin)?.into_output();

    sensor_pin.set_high();
    sleep(Duration::from_secs(5)); // breathing time

    let mut plant_state: Vec<(Plant, SoilHumidityReading)> = vec![];
    for plant in setup.plants {
        let result: f32 = normalize_mcp_readings(
            mcp3008.read_adc(plant.analog_channel)?,
            &plant.sensor_params,
        );

        plant_state.push((plant, SoilHumidityReading { humidity: result }))
    }

    let water_lever_reading = mcp3008.read_adc(setup.water_level_sensor.analog_channel)?;

    sensor_pin.set_low();

    Ok(PlantsState {
        plants: plant_state,
        water_container_reading: calculate_water_level(water_lever_reading),
    })
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // Create a connection pool to database
    let pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let path = env::var("CONFIG_PATH").unwrap_or("/home/pi/config.json".to_string());
    let setup = settings_from_json(&path)?;
    let state = read_plants_state(setup).await?;
    println!("Water container level: {:?}", state.water_container_reading);

    let water_container_empty = matches!(
        state.water_container_reading,
        WaterLevelReading::NoWaterContact
    );

    if water_container_empty {
        mail::send_alert_mail()?;
    }

    for (plant, reading) in state.plants {
        let is_dry = reading.humidity < plant.watering_params.requires_watering_level;
        let is_time_to_water = plant.watering_params.should_be_watered();

        sqlx::query(
            "insert into readings (time, sensor, metric, value) values ( now(), $1, 'humidity', $2 )",
        )
        .bind(&plant.id)
        .bind(&reading.humidity)
        .execute(&pool).await?;

        println!(
            "plant {} STATUS is_dry ({}) -> {} time_to_water -> {} water_container_empty  -> {}",
            plant.name, reading.humidity, is_dry, is_time_to_water, water_container_empty
        );

        if is_dry && is_time_to_water && !water_container_empty {
            println!("I will water {}", plant.name);
            let mut pump_pin = Gpio::new()?
                .get(plant.watering_params.pump_gpio)?
                .into_output();
            pump_pin.reset_on_drop();
            pump_pin.set_high();
            sleep(Duration::from_secs(
                plant.watering_params.water_for_seconds as u64,
            ));
            pump_pin.set_low();
            sqlx::query(
                        "insert into water_history (time, sensor,duration_seconds) values ( now(), $1, $2 )")
            .bind(plant.id)
            .bind(plant.watering_params.water_for_seconds)
            .execute(&pool).await?;
        } else {
            println!("skipping watering plant {}", plant.name);
        }
    }

    Ok(())
}
