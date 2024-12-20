use crate::handlers::kanban::list::{create_list, delete_list, update_list};
use axum::{routing::post, routing::put, Router};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_list))
        .route("/:list_id", put(update_list).delete(delete_list))
}
