extern crate log;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use actix_cors::Cors;

use sqlx::PgPool;
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;


use actix_web::middleware::Logger;
use env_logger::Env;


use serde::{Deserialize, Serialize};

// todo implement Responder  according to https://actix.rs/docs/handlers/
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ReadingDb {
    time: chrono::NaiveDateTime,
    sensor: String,
    metric: String,
    value: f32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
//insert into water_history (time, sensor,duration_seconds) values ( now(), $1, $2 )
pub struct WateringTimeRecordings {
    time: chrono::NaiveDateTime,
    sensor: String,
    duration_seconds: i32,
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

async fn get_dashboard(conn: &sqlx::Pool<Postgres>, info: &Info) -> Result<WateringDashboard> {
    let now = chrono::offset::Local::now().naive_local();
    // todo add validation that start_date < finish_date
    let start_date = info
        .start
        .as_ref()
        .map(|d| chrono::NaiveDateTime::from_timestamp(d.parse::<i64>().unwrap(), 0_u32))
        .unwrap_or(now - chrono::Duration::days(5));

    let finish_date = info
        .finish
        .as_ref()
        .map(|d| chrono::NaiveDateTime::from_timestamp(d.parse::<i64>().unwrap(), 0_u32))
        .unwrap_or(now);
    if start_date >= finish_date {
        return Err(anyhow::anyhow!("start_date cannot be > then finish_date"));
    }

    let readings: Vec<ReadingDb> =
        sqlx::query_as::<_, ReadingDb>("select * from readings where time > $1 and time < $2")
            .bind(start_date)
            .bind(finish_date)
            .fetch_all(&*conn)
            .await?;

    let waterings: Vec<WateringTimeRecordings> = sqlx::query_as::<_, WateringTimeRecordings>(
        "select * from water_history where time > $1 and time < $2",
    )
    .bind(start_date)
    .bind(finish_date)
    .fetch_all(&*conn)
    .await?;

    let dashboard = WateringDashboard {
        from: start_date,
        to: finish_date,
        sensor_readings: readings,
        waterings: waterings,
    };

    Ok(dashboard)
}

#[get("/sensors/readings")]
async fn index(db_pool: web::Data<PgPool>, info: web::Query<Info>) -> impl Responder {
    let conn = db_pool.get_ref();

    match get_dashboard(conn, &info).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    // todo add error handler
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin("http://127.0.0.1:8081");

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(pool.clone()) // pass database pool to application so we can access it inside handlers
            .service(index)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await?;
    Ok(())
}
