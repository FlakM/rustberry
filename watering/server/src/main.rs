extern crate log;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use dotenv::dotenv;
use sqlx::PgPool;
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;

use actix_web::middleware::Logger;
use env_logger::Env;

use serde::{Deserialize, Serialize};

mod rest;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ReadingDb {
    time: chrono::DateTime<chrono::offset::Local>,
    sensor: String,
    metric: String,
    value: f32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
//insert into water_history (time, sensor,duration_seconds) values ( now(), $1, $2 )
pub struct WateringTimeRecordings {
    time: chrono::DateTime<chrono::offset::Local>,
    sensor: String,
    duration_seconds: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct Info {
    // todo rename fields and make them longs?
    pub start: Option<String>,
    pub finish: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WateringDashboard {
    pub from: chrono::DateTime<chrono::offset::Local>,
    pub to: chrono::DateTime<chrono::offset::Local>,
    pub sensor_readings: Vec<ReadingDb>,
    pub waterings: Vec<WateringTimeRecordings>,
}

async fn get_dashboard(
    conn: &sqlx::Pool<Postgres>,
    info: &Info,
) -> Result<rest::WateringChartData> {
    let now = chrono::offset::Local::now();
    // todo add validation that start_date < finish_date
    let start_date = info
        .start
        .as_ref()
        .map(|d| {
            let timestamp = d.parse::<i64>().unwrap();
            let naive = NaiveDateTime::from_timestamp(timestamp, 0);
            let utc = DateTime::<chrono::Utc>::from_utc(naive, Utc);
            let local: DateTime<Local> = DateTime::from(utc);
            local
        })
        .unwrap_or(now - chrono::Duration::days(5));

    let finish_date = info
        .finish
        .as_ref()
        .map(|d| {
            let timestamp = d.parse::<i64>().unwrap();
            let naive = NaiveDateTime::from_timestamp(timestamp, 0);
            let utc = DateTime::<chrono::Utc>::from_utc(naive, Utc);
            let local: DateTime<Local> = DateTime::from(utc);
            local
        })
        .unwrap_or(now);
    if start_date >= finish_date {
        return Err(anyhow::anyhow!("start_date cannot be > then finish_date"));
    }

    let readings: Vec<ReadingDb> =
        sqlx::query_as::<_, ReadingDb>("select * from readings r join sensors s on s.id  = r.sensor where r.time > $1 and r.time < $2")
            .bind(start_date)
            .bind(finish_date)
            .fetch_all(&*conn)
            .await?;

    let waterings: Vec<WateringTimeRecordings> = sqlx::query_as::<_, WateringTimeRecordings>(
        "select * from water_history h join sensors s on s.id  = h  .sensor where time > $1 and time < $2",
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

    Ok(rest::WateringChartData::from_dashboard(&dashboard)?)
}

#[get("/sensors/readings")]
async fn index(db_pool: web::Data<PgPool>, info: web::Query<Info>) -> impl Responder {
    let conn = db_pool.get_ref();

    match get_dashboard(conn, &info).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

#[get("/")]
async fn actual_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
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
            .allowed_origin("http://192.168.0.201:8080");

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(pool.clone()) // pass database pool to application so we can access it inside handlers
            .service(index)
            .service(actual_index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
