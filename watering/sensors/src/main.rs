// extern crate openssl;

use models::*;
use rppal::gpio::Gpio;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use mcp3008::Mcp3008;

mod mail;

async fn read_plants_state(
    setup: Setup,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> anyhow::Result<PlantsState> {
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0")?;
    let mut sensor_pin = Gpio::new()?.get(setup.sensor_power_pin)?.into_output();

    sensor_pin.set_high();
    sleep(Duration::from_secs(1)); // breathing time

    let mut plant_state: Vec<(Plant, SoilHumidityReading)> = vec![];
    for plant in setup.plants {
        let result: f32 = normalize_mcp_readings(
            mcp3008.read_adc(plant.analog_channel)?,
            &plant.sensor_params,
        );
        sqlx::query!(
            "insert into readings (time, sensor, metric, value) values ( now(), $1, 'humidity', $2 )",
            plant.id,
            result
        )
        .execute(pool).await?;

        println!("Water level for plant {} {}% updated", plant.name, result);
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
    let state = read_plants_state(setup, &pool).await?;
    println!("Water container level: {:?}", state.water_container_reading);

    if matches!(
        state.water_container_reading,
        WaterLevelReading::NoWaterContact
    ) {
        mail::send_alert_mail()?;
        return Err(anyhow::anyhow!("Refill water container"));
    }

    for (plant, reading) in state.plants {
        let is_dry = reading.humidity < plant.watering_params.requires_watering_level;
        let is_time_to_water = plant.watering_params.should_be_watered();
        if is_dry && is_time_to_water {
            println!("I will water {}", plant.name);
            let mut pump_pin = Gpio::new()?
                .get(plant.watering_params.pump_gpio)?
                .into_output();
            pump_pin.reset_on_drop();
            pump_pin.set_high();
            sleep(Duration::from_secs(
                plant.watering_params.water_for_seconds as u64,
            ));
            pump_pin.is_set_low();
            sqlx::query!(
                        "insert into water_history (time, sensor,duration_seconds) values ( now(), $1, $2 )",
                        plant.id,
                        plant.watering_params.water_for_seconds
                    )
                    .execute(&pool).await?;
        } else {
            println!(
                "skipping watering plant {} is_dry ({}) -> {} time_to_water -> {}",
                plant.name, reading.humidity, is_dry, is_time_to_water
            );
        }
    }

    Ok(())
}
