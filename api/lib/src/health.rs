use axum::extract::State;
use std::sync::Arc;

pub async fn version(State(pool): State<Arc<sqlx::PgPool>>) -> String {
    tracing::info!("Getting version");
    sqlx::query_scalar::<sqlx::Postgres, String>("SELECT version()")
        .fetch_one(pool.as_ref())
        .await
        .unwrap_or_else(|err| format!("Error: {:?}", err))
}
