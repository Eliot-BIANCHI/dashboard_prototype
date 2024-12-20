use crate::handlers::kanban::{
    create_kanban, delete_kanban, delete_user_from_kanban, get_kanban, get_kanbans_overviews,
    update_kanban,
};
use axum::{
    routing::{delete, get},
    Router,
};
use sqlx::PgPool;

pub mod list;
pub mod task;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_kanbans_overviews).post(create_kanban))
        .route(
            "/:kanban_id",
            get(get_kanban).put(update_kanban).delete(delete_kanban),
        )
        .route(
            "/:kanban_id/shared/:user_id",
            delete(delete_user_from_kanban),
        )
}
