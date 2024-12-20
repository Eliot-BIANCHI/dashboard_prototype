use crate::handlers::invitation::kanban::{
    accept_kanban_invitation, create_kanban_invitation, reject_kanban_invitation,
};
use axum::{routing::post, Router};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_kanban_invitation))
        .route("/:invitation_id/accept", post(accept_kanban_invitation))
        .route("/:invitation_id/reject", post(reject_kanban_invitation))
}
