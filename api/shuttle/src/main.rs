use std::sync::Arc;

use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn main(#[Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize the database if not already initialized
    tracing::info!("Migrating database");
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = Arc::new(pool);
    let router = Router::new().route("/version", get(api_lib::version)).with_state(state);
    Ok(router.into())
}
