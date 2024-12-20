use crate::handlers::user::auth::{auto_log_in, log_in, log_out};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/auto-log-in", get(auto_log_in))
        .route("/log-in", post(log_in))
        .route("/log-out", get(log_out))
}
