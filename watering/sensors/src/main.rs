// extern crate openssl;

use clap::Clap;
use models::*;
use rppal::gpio::Gpio;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use mcp3008::Mcp3008;

mod mail;

/// This is a small cli tool for plant watering automation
#[derive(Clap)]
#[clap(version = "1.0", author = "FlakM <maciej.jan.flak@gmail.com>")]
struct Opts {
    /// Sets a custom config file
    #[clap(short, long, default_value = "/home/pi/config.json")]
    config_path: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.3", author = "FlakM <maciej.jan.flak@gmail.com>")]
    CheckLevels(CheckLevels),
    WaterPlants(WaterPlants),
}

/// A subcommand for checking water levels
#[derive(Clap)]
struct CheckLevels {}

/// A subcommand for watering plants
#[derive(Clap)]
struct WaterPlants {}

async fn read_plants_state(
    setup: Setup,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> anyhow::Result<PlantsState> {
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();

    let mut sensor_pin = Gpio::new()?.get(setup.sensor_power_pin)?.into_output();

    sensor_pin.set_high();
    sleep(Duration::from_secs(5));

    let mut plant_state: Vec<(Plant, SoilHumidityReading)> = vec![];

    for plant in setup.plants {
        let result: f32 = calculate_percentage(
            mcp3008.read_adc(plant.analog_channel).unwrap(),
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

    let water_lever_reading = mcp3008
        .read_adc(setup.water_level_sensor.analog_channel)
        .unwrap();

    sensor_pin.set_low();

    Ok(PlantsState {
        plants: plant_state,
        // todo implement this when the sensor is under water protected by epoxy resin
        water_container_reading: calculate_water_level(water_lever_reading),
    })
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1_u32)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    match opts.subcmd {
        SubCommand::WaterPlants(_cmd) => {
            let setup = settings_from_json(&opts.config_path)?;
            let state = read_plants_state(setup, &pool).await?;
            println!("Water container level: {:?}", state.water_container_reading);

            match state.water_container_reading {
                WaterLevelReading::CompleteUnderWater => (),
                _ => {
                    mail::send_mail(
                        &env::var("MAIL_TO")?,
                        &env::var("SMTP_USERNAME")?,
                        &env::var("SMTP_PASSWORD")?,
                    )?;
                    return Err(anyhow::anyhow!("Refill water container"));
                }
            };

            for (plant, reading) in state.plants {
                let is_dry = reading.humidity < plant.watering_params.requires_watering_level;
                let is_time_to_water = plant.watering_params.should_be_watered();
                if is_dry && is_time_to_water {
                    println!("i will water plant");
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
        }
        SubCommand::CheckLevels(_cmd) => {
            let setup = settings_from_json(&opts.config_path)?;
            let state = read_plants_state(setup, &pool).await?;
            println!("read config from path {}", &opts.config_path);

            println!(
                "Water level in the tank {:?}",
                state.water_container_reading
            );
        }
    }

    Ok(())
}
