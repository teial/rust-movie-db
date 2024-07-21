use std::sync::Arc;

use axum::{extract::State, routing::get, Router};
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn main(#[Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = Arc::new(pool);
    let router = Router::new().route("/version", get(version)).with_state(state);
    Ok(router.into())
}

async fn version(State(pool): State<Arc<sqlx::PgPool>>) -> String {
    sqlx::query_scalar::<sqlx::Postgres, String>("SELECT version()")
        .fetch_one(pool.as_ref())
        .await
        .unwrap_or_else(|err| format!("Error: {:?}", err))
}
