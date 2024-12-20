use crate::handlers::role_permission::permission::create_permission;
use axum::{routing::post, Router};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new().route("/", post(create_permission))
}
