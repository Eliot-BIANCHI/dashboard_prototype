use crate::models::role_permission::{
    CalendarRolePermission, KanbanRole, KanbanRolePermission, Permission, Role, RolePermission,
};
use crate::utils::authorization::{has_permission_to, Action, Table};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use serde_json::json;
use sqlx::PgPool;

pub mod permission;
pub mod role;

pub async fn get_roles_permissions(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_roles_permissions\n", "HANDLER");

    has_permission_to(&[Action::ViewAll], Table::RolePermission, user_id, &pg_pool).await?;

    let roles = sqlx::query_as!(Role, "SELECT id, label FROM role;",)
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    let permissions = sqlx::query_as!(
        Permission,
        "
        SELECT id, label FROM permission
        LEFT JOIN kanban_role_permission ON kanban_role_permission.permission_id = permission.id
        LEFT JOIN calendar_role_permission ON calendar_role_permission.permission_id = permission.id
        WHERE kanban_role_permission.permission_id IS NULL AND calendar_role_permission.permission_id IS NULL
        ORDER BY permission.id;
        ",
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let roles_permissions = sqlx::query_as!(
        RolePermission,
        "SELECT role_id, permission_id FROM role_permission;",
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
        json!({
            "success": true,
            "data": {
                "roles": roles,
                "permissions": permissions,
                "rolesPermissions": roles_permissions,
            }
        })
        .to_string(),
    ))
}

pub async fn get_calendar_roles_permissions(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_calendar_roles_permissions\n", "HANDLER");

    has_permission_to(&[Action::ViewAll], Table::RolePermission, user_id, &pg_pool).await?;

    let calendar_roles = sqlx::query_as!(
        KanbanRole,
        "SELECT id, label, is_default FROM calendar_role;",
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let calendar_permissions = sqlx::query_as!(
        Permission,
        "
            SELECT DISTINCT id, label FROM permission
            JOIN calendar_role_permission ON calendar_role_permission.permission_id = permission.id
            ORDER BY permission.id;
        ",
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let calendar_roles_permissions = sqlx::query_as!(
        CalendarRolePermission,
        "SELECT calendar_role_id, permission_id FROM calendar_role_permission;",
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
        json!({
            "success": true,
            "data": {
                "calendarRoles": calendar_roles,
                "calendarPermissions": calendar_permissions,
                "calendarRolesPermissions": calendar_roles_permissions,
            }
        })
        .to_string(),
    ))
}

pub async fn get_kanbans_roles_permissions(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_kanbans_roles_permissions\n", "HANDLER");

    has_permission_to(&[Action::ViewAll], Table::RolePermission, user_id, &pg_pool).await?;

    let kanban_roles =
        sqlx::query_as!(KanbanRole, "SELECT id, label, is_default FROM kanban_role;",)
            .fetch_all(&pg_pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "success": false, "message": e.to_string() }).to_string(),
                )
            })?;

    let kanban_permissions = sqlx::query_as!(
        Permission,
        "
        SELECT DISTINCT id, label FROM permission
        JOIN kanban_role_permission ON kanban_role_permission.permission_id = permission.id
        ORDER BY permission.id;
        ",
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let kanban_roles_permissions = sqlx::query_as!(
        KanbanRolePermission,
        "SELECT kanban_role_id, permission_id FROM kanban_role_permission;",
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
        json!({
            "success": true,
            "data": {
                "kanbanRoles": kanban_roles,
                "kanbanPermissions": kanban_permissions,
                "kanbanRolesPermissions": kanban_roles_permissions,
            }
        })
        .to_string(),
    ))
}

pub async fn create_role_permission(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((role_id, permission_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_role_permission\n", "HANDLER");

    has_permission_to(&[Action::Create], Table::RolePermission, user_id, &pg_pool).await?;

    sqlx::query!(
        "
        INSERT INTO role_permission (role_id, permission_id) VALUES ($1, $2);
        ",
        role_id,
        permission_id
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({ "success": true }).to_string()))
}

pub async fn delete_role_permission(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((role_id, permission_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_role_permission\n", "HANDLER");

    has_permission_to(
        &[Action::DeleteAll],
        Table::RolePermission,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        DELETE FROM role_permission WHERE role_id = $1 AND permission_id = $2;
        ",
        role_id,
        permission_id
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({ "success": true }).to_string()))
}

pub async fn create_kanban_role_permission(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((kanban_role_id, permission_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_kanban_role_permission\n", "HANDLER");

    has_permission_to(&[Action::Create], Table::RolePermission, user_id, &pg_pool).await?;

    sqlx::query!(
        "
        INSERT INTO kanban_role_permission (kanban_role_id, permission_id) VALUES ($1, $2);
        ",
        kanban_role_id,
        permission_id
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({ "success": true }).to_string()))
}

pub async fn delete_kanban_role_permission(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((kanban_role_id, permission_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_kanban_role_permission\n", "HANDLER");

    has_permission_to(
        &[Action::DeleteAll],
        Table::RolePermission,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        DELETE FROM kanban_role_permission WHERE kanban_role_id = $1 AND permission_id = $2;
        ",
        kanban_role_id,
        permission_id
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({ "success": true }).to_string()))
}
