use crate::handlers::user::account::{get_account, update_account};
use axum::{routing::get, Router};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new().route("/", get(get_account).put(update_account))
}
