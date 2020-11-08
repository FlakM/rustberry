// extern crate openssl;

use clap::Clap;
use rppal::gpio::Gpio;
use rustberry::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::BigDecimal;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use mcp3008::Mcp3008;

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
struct CheckLevels {
    /// Print information to std out
    #[clap(short)]
    debug: bool,
}

/// A subcommand for watering plants
#[derive(Clap)]
struct WaterPlants {}

fn settings_from_json(path_to_config: &str) -> anyhow::Result<Setup> {
    let content = std::fs::read_to_string(path_to_config)?;
    let setup = Setup::from_str(&content)?;
    Ok(setup)
}

fn read_plants_state(setup: Setup) -> anyhow::Result<PlantsState> {
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();

    let plants_state: Vec<(Plant, SoilHumidityReading)> = setup
        .plants
        .into_iter()
        .map(|plant| {
            let reading = mcp3008.read_adc(plant.analog_channel).unwrap();
            let result: f64 = calculate_percentage(reading, &plant.sensor_params);
            (plant, SoilHumidityReading { humidity: result })
        })
        .collect();

    Ok(PlantsState {
        plants: plants_state,
        // todo implement this when the sensor is under water protected by epoxy resin
        water_container_reading: WaterLevelReading::NotComplete,
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
        SubCommand::WaterPlants(cmd) => {
            let setup = settings_from_json(&opts.config_path)?;
            let state = read_plants_state(setup)?;

            match state.water_container_reading {
                WaterLevelReading::CompleteUnderWater => (),
                WaterLevelReading::NotComplete => eprintln!("Water container might require refill"),
                _ => return Err(anyhow::anyhow!("Refill water container")),
            };

            for (plant, reading) in state.plants {
                if reading.humidity < plant.requires_watering_level {
                    println!("i will water plant");
                    let mut pump_pin = Gpio::new()?.get(plant.pump_gpio)?.into_output();
                    pump_pin.reset_on_drop();
                    pump_pin.set_high();
                    sleep(Duration::from_secs(plant.water_for_seconds));
                    pump_pin.is_set_low();
                    sqlx::query!(
                        "insert into water_history (time, sensor,duration_seconds) values ( now(), $1, $2 )",
                        plant.id,
                        BigDecimal::from(plant.water_for_seconds)
                    )
                    .execute(&pool).await?;
                }

            }
            todo!();
        }
        SubCommand::CheckLevels(cmd) => {
            let setup = settings_from_json(&opts.config_path)?;
            let state = read_plants_state(setup)?;

            for (plant, reading) in state.plants.iter() {
                let result_decimal = BigDecimal::from(reading.humidity);
                // Insert the task, then obtain the ID of this row
                let id = sqlx::query!(
                    "insert into readings (time, sensor, metric, value) values ( now(), $1, 'humidity', $2 )",
                    plant.id,
                    result_decimal
                )
                .execute(&pool).await?;

                println!(
                    "Water level for plant {} {}% updated as {:?}",
                    plant.name, reading.humidity, id
                );
            }

         
        }
    }

    Ok(())
}
