use crate::handlers::invitation::{
    get_pending_invitations, get_received_invitations, get_sent_invitations,
};
use axum::{routing::get, Router};
use sqlx::PgPool;

pub mod kanban;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_pending_invitations))
        .route("/received", get(get_received_invitations))
        .route("/sent", get(get_sent_invitations))
}
