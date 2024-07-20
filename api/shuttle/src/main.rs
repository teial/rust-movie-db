use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::Executor;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let router = Router::new().route("/", get(hello_world));
    Ok(router.into())
}
