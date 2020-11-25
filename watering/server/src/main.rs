use std::env;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    use sqlx::prelude::*;

    use tide_sqlx::PostgresConnectionMiddleware;
    use tide_sqlx::PostgresRequestExt;

    let mut app = tide::new();

    let url = env::var("DATABASE_URL")?;

    app.with(PostgresConnectionMiddleware::new(&url, 1).await?);

    app.at("/").get(|req: tide::Request<()>| async move {
        let mut pg_conn = req.postgres_conn().await;

        pg_conn.acquire().await?; // Pass this to e.g. "fetch_optional()" from a sqlx::Query

        Ok("hi")
    });
    

    app.listen("127.0.0.1:8080").await?;
    Ok(())


}