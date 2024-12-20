use crate::handlers::role_permission::{
    create_kanban_role_permission, create_role_permission, delete_kanban_role_permission,
    delete_role_permission, get_calendar_roles_permissions, get_kanbans_roles_permissions,
    get_roles_permissions,
};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

pub mod permission;
pub mod role;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_roles_permissions))
        .route("/calendars", get(get_calendar_roles_permissions))
        .route("/kanbans", get(get_kanbans_roles_permissions))
        .route(
            "/:role_id/:permission_id",
            post(create_role_permission).delete(delete_role_permission),
        )
        .route(
            "/kanbans/:kanban_role_id/:permission_id",
            post(create_kanban_role_permission).delete(delete_kanban_role_permission),
        )
}
