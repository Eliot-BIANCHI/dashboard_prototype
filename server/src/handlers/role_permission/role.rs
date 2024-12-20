use crate::models::role_permission::role::CreateRoleReq;
use crate::models::role_permission::Role;
use crate::utils::authorization::{has_permission_to, Action, Table};
use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::json;
use sqlx::PgPool;

pub async fn get_roles(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_kanban_roles\n", "HANDLER");

    let roles = sqlx::query_as!(Role, "SELECT id, label FROM role ORDER BY id DESC;",)
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": { "roles": roles } }).to_string(),
    ))
}

pub async fn get_kanban_roles(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_kanban_roles\n", "HANDLER");

    let kanban_roles =
        sqlx::query_as!(Role, "SELECT id, label FROM kanban_role ORDER BY id DESC;",)
            .fetch_all(&pg_pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "success": false, "message": e.to_string() }).to_string(),
                )
            })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": { "kanbanRoles": kanban_roles } }).to_string(),
    ))
}

pub async fn get_calendar_roles(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_calendar_roles\n", "HANDLER");

    let calendar_roles = sqlx::query_as!(
        Role,
        "SELECT id, label FROM calendar_role ORDER BY id DESC;",
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": { "calendarRoles": calendar_roles } }).to_string(),
    ))
}

pub async fn create_role(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateRoleReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_role\n", "HANDLER");

    has_permission_to(&[Action::Create], Table::Role, user_id, &pg_pool).await?;

    let role_id = sqlx::query_scalar!(
        "INSERT INTO role (label) VALUES ($1) RETURNING id",
        credentials.label,
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": { "roleId": role_id } }).to_string(),
    ))
}
