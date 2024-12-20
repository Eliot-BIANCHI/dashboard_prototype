use crate::handlers::calendar::schedule::{create_schedule, delete_schedule, update_schedule};
use axum::{routing::post, routing::put, Router};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new().route("/", post(create_schedule)).route(
        "/:schedule_id",
        put(update_schedule).delete(delete_schedule),
    )
}
