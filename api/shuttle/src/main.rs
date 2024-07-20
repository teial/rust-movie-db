use axum::{routing::get, Router};
use shuttle_shared_db::Postgres;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[Postgres] _pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    Ok(router.into())
}
