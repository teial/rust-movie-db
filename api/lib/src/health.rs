use axum::{extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn health_check(State(pool): State<Arc<sqlx::PgPool>>) -> impl IntoResponse {
    tracing::info!("Health check");
    match version(pool.as_ref()).await {
        Ok(version) => (StatusCode::OK, [("version", version)]),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, [("error", err.to_string())]),
    }
}

async fn version(pool: &sqlx::PgPool) -> Result<String, sqlx::Error> {
    sqlx::query_scalar::<sqlx::Postgres, String>("SELECT version()")
        .fetch_one(pool)
        .await
}
