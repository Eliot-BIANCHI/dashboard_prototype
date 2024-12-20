use crate::models::kanban::list::{CreateListReq, UpdateListReq};
use crate::utils::kanban_authorization::{has_kanban_permission_to, KanbanAction, KanbanTable};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;
use sqlx::PgPool;

pub async fn create_list(
    Extension(user_id): Extension<i32>,
    Path(kanban_id): Path<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateListReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_list\n", "HANDLER");

    has_kanban_permission_to(
        &[KanbanAction::Create],
        KanbanTable::List,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    let list_id = sqlx::query_scalar!(
        "INSERT INTO list (label, kanban_id, owner_id) VALUES ($1, $2, $3) RETURNING id;",
        credentials.label,
        kanban_id,
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
        json!({ "success": true, "data": { "listId": list_id } }).to_string(),
    ))
}

pub async fn update_list(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((kanban_id, list_id)): Path<(i32, i32)>,
    Json(credentials): Json<UpdateListReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - update_list\n", "HANDLER");

    has_kanban_permission_to(
        &[
            KanbanAction::UpdateOwn(user_id, list_id),
            KanbanAction::UpdateAll,
        ],
        KanbanTable::List,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        UPDATE list SET
            label = $2
        WHERE id = $1
        ",
        list_id,
        credentials.label,
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

pub async fn delete_list(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((kanban_id, list_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_list\n", "HANDLER");

    has_kanban_permission_to(
        &[
            KanbanAction::DeleteOwn(user_id, list_id),
            KanbanAction::DeleteAll,
        ],
        KanbanTable::List,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!("DELETE FROM list WHERE id = $1", list_id)
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
