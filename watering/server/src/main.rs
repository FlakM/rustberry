use std::env;

use serde::{Deserialize, Serialize};
use sqlx::PgConnection;


use {
    tracing::Level,
};

#[derive(Serialize, Deserialize, Debug)]

struct Readings {
    readings: Vec<ReadingDb>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct ReadingDb {
    time: chrono::DateTime<chrono::Utc>,
    sensor: String,
    metric: String,
    value: f32,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    use sqlx::prelude::*;

    use tide_sqlx::PostgresConnectionMiddleware;
    use tide_sqlx::PostgresRequestExt;

    tide::log::with_level(tide::log::LevelFilter::Info);


    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");


    let mut app = tide::new();

    let url = env::var("DATABASE_URL")?;

    app.with(PostgresConnectionMiddleware::new(&url, 1).await?);
    app.with(tide_tracing::TraceMiddleware::new());

    app.at("/sensors/readings")
        .get(|req: tide::Request<()>| async move {
            let mut pg_conn = req.postgres_conn().await;
            let conn: &mut PgConnection = pg_conn.acquire().await?;
            let now = chrono::offset::Utc::now().naive_local();
            tracing::info!("{:?}", req);

            let start_date = req
                .param("start")
                .map(|d| chrono::NaiveDateTime::from_timestamp(d.parse::<i64>().unwrap(), 0_u32))
                .unwrap_or(now - chrono::Duration::days(5));

            let finish_date = req
                .param("finish")
                .map(|d| chrono::NaiveDateTime::from_timestamp(d.parse::<i64>().unwrap(), 0_u32))
                .unwrap_or(now);


            tracing::info!("selecting range {} to {}", start_date, finish_date);


            let stream = sqlx::query_as::<_, ReadingDb>(
                "select * from readings where time > $1 and time < $2",
            )
            .bind(start_date)
            .bind(finish_date)
            .fetch_all(conn)
            .await?;

            Ok(tide::Body::from_json(&stream)?)
        });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
