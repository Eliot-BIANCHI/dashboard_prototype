use crate::handlers::role_permission::role::{
    create_role, get_calendar_roles, get_kanban_roles, get_roles,
};
use axum::{routing::get, Router};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_roles).post(create_role))
        .route("/calendars", get(get_calendar_roles))
        .route("/kanbans", get(get_kanban_roles))
}
