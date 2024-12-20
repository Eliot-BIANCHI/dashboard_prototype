use crate::models::kanban::task::{
    AssignUser, CreateTaskReq, PatchTaskCompletionReq, PatchTaskListReq, UpdateTaskReq,
};
use crate::utils::kanban_authorization::{has_kanban_permission_to, KanbanAction, KanbanTable};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;
use sqlx::PgPool;

pub async fn create_task(
    Extension(user_id): Extension<i32>,
    Path((kanban_id, list_id)): Path<(i32, i32)>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_task\n", "HANDLER");

    has_kanban_permission_to(
        &[KanbanAction::Create],
        KanbanTable::Task,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    let task_id = sqlx::query_scalar!(
        "
        INSERT INTO task (label, completion_id, list_id, priority, owner_id)
        VALUES ($1, $2, $3, $4, $5) RETURNING id
        ",
        credentials.label,
        credentials.completion_id,
        list_id,
        credentials.priority,
        user_id
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
        json!({ "success": true, "data": { "taskId": task_id } }).to_string(),
    ))
}

pub async fn update_task(
    Extension(user_id): Extension<i32>,
    Path((kanban_id, _, task_id)): Path<(i32, i32, i32)>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<UpdateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - update_task\n", "HANDLER");

    has_kanban_permission_to(
        &[
            KanbanAction::UpdateOwn(user_id, task_id),
            KanbanAction::UpdateAll,
        ],
        KanbanTable::Task,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        UPDATE task SET
            label = $2,
            completion_id = $3,
            priority = $4
        WHERE id = $1
        ",
        task_id,
        credentials.label,
        credentials.completion_id,
        credentials.priority,
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

pub async fn patch_task_list(
    Extension(user_id): Extension<i32>,
    Path((kanban_id, _, task_id)): Path<(i32, i32, i32)>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<PatchTaskListReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - patch_task_list\n", "HANDLER");

    has_kanban_permission_to(
        &[
            KanbanAction::MoveOwn(user_id, task_id),
            KanbanAction::MoveAll,
        ],
        KanbanTable::Task,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "UPDATE task SET list_id = $2 WHERE id = $1",
        task_id,
        credentials.list_id
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

pub async fn patch_task_completion(
    Extension(user_id): Extension<i32>,
    Path((kanban_id, _, task_id)): Path<(i32, i32, i32)>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<PatchTaskCompletionReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - patch_task_completion\n", "HANDLER");

    has_kanban_permission_to(
        &[
            KanbanAction::UpdateOwn(user_id, task_id),
            KanbanAction::UpdateAll,
        ],
        KanbanTable::Task,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "UPDATE task SET completion_id = $2 WHERE id = $1",
        task_id,
        credentials.completion_id
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

pub async fn delete_task(
    Extension(user_id): Extension<i32>,
    Path((kanban_id, _, task_id)): Path<(i32, i32, i32)>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_task\n", "HANDLER");

    has_kanban_permission_to(
        &[
            KanbanAction::DeleteOwn(user_id, task_id),
            KanbanAction::DeleteAll,
        ],
        KanbanTable::Task,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!("DELETE FROM task WHERE id = $1", task_id)
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

pub async fn assign_user(
    Extension(user_id): Extension<i32>,
    Path((kanban_id, _, task_id)): Path<(i32, i32, i32)>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<AssignUser>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - assign_user\n", "HANDLER");

    has_kanban_permission_to(
        &[KanbanAction::AssignSelf],
        KanbanTable::Task,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        INSERT INTO task_assignee (id, user_id)
        VALUES ($1, $2);
        ",
        task_id,
        credentials.user_id
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

pub async fn unassign_user(
    Extension(user_id): Extension<i32>,
    Path((kanban_id, _, task_id)): Path<(i32, i32, i32)>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<AssignUser>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - assign_user\n", "HANDLER");

    has_kanban_permission_to(
        &[KanbanAction::UnassignSelf],
        KanbanTable::Task,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        DELETE FROM task_assignee WHERE id = $1 AND user_id = $2;
        ",
        task_id,
        credentials.user_id
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
