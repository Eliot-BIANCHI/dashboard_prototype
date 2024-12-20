use crate::models::kanban::list::List;
use crate::models::kanban::task::{Task, TaskAssignee};
use crate::models::kanban::{CreateKanbanReq, Overview, UpdateKanbanReq};
use crate::models::user::KanbanUser;
use crate::utils::authorization::{has_permission_to, Action, Table};
use crate::utils::kanban_authorization::{has_kanban_permission_to, KanbanAction, KanbanTable};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;
use sqlx::PgPool;

pub mod list;
pub mod task;

pub async fn get_kanbans_overviews(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_kanbans_overviews\n", "HANDLER");

    let rows = sqlx::query_as!(
        Overview,
        "
        SELECT id, label, is_shared FROM kanban WHERE owner_id = $1
        OR kanban.id IN (
            SELECT shared_kanban.id
            FROM shared_kanban
            WHERE shared_kanban.user_id = $1
        );
        ",
        user_id
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
        json!({ "success": true, "data": { "overviews": rows } }).to_string(),
    ))
}

pub async fn get_kanban(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(kanban_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_kanban\n", "HANDLER");

    has_permission_to(
        &[
            Action::ViewOwn(user_id, kanban_id),
            Action::ViewInvitedTo(user_id, kanban_id),
            Action::ViewAll,
        ],
        Table::Kanban,
        user_id,
        &pg_pool,
    )
    .await?;

    let lists = sqlx::query_as!(
        List,
        "
        SELECT
            id, label, owner_id
        FROM list
        WHERE list.kanban_id = $1;
        ",
        kanban_id
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let lists_ids: Vec<i32> = lists.iter().map(|list| list.id).collect();

    let tasks = sqlx::query_as!(
        Task,
        "
        SELECT
            task.id,
            task.label,
            task.completion_id,
            task.owner_id,
            list.id AS list_id,
            task.created_at,
            task.priority
        FROM task
        JOIN list ON list.id = task.list_id
        WHERE task.list_id IN (SELECT unnest($1::integer[]));
        ",
        &lists_ids
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let owner = sqlx::query_as!(
        KanbanUser,
        "
            SELECT
                app_user.id,
                first_name,
                last_name,
                image_url,
                kanban_role.id as kanban_role_id,
                kanban_role.label as kanban_role_label
            FROM kanban
            INNER JOIN app_user ON kanban.owner_id = app_user.id
            JOIN user_kanban_role
                ON user_kanban_role.user_id = app_user.id
                AND user_kanban_role.kanban_id = kanban.id
            JOIN kanban_role ON kanban_role.id = user_kanban_role.kanban_role_id
            WHERE kanban.id = $1;
        ",
        kanban_id,
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let is_shared: bool = sqlx::query_scalar!(
        "
            SELECT
                is_shared
            FROM kanban
            INNER JOIN app_user
            ON kanban.owner_id = app_user.id
            WHERE kanban.id = $1;
        ",
        kanban_id,
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let user_permissions = sqlx::query_scalar!(
        "
        SELECT
            permission.label
        FROM user_kanban_role
        JOIN kanban_role_permission ON kanban_role_permission.kanban_role_id = user_kanban_role.kanban_role_id
        JOIN permission ON permission.id = kanban_role_permission.permission_id
        WHERE user_kanban_role.user_id = $1 AND user_kanban_role.kanban_id = $2;
        ",
        user_id,
        kanban_id
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    let shared_with;
    let tasks_assignees;

    if is_shared {
        shared_with = sqlx::query_as!(
            KanbanUser,
            "
            SELECT
                app_user.id,
                app_user.first_name,
                app_user.last_name,
                app_user.image_url,
                kanban_role.id as kanban_role_id,
                kanban_role.label as kanban_role_label
            FROM shared_kanban
            JOIN app_user ON app_user.id = shared_kanban.user_id
            JOIN user_kanban_role
                ON user_kanban_role.user_id = app_user.id
                AND user_kanban_role.kanban_id = shared_kanban.id
            JOIN kanban_role ON kanban_role.id = user_kanban_role.kanban_role_id
            WHERE shared_kanban.id = $1;
            ",
            kanban_id
        )
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

        let tasks_ids: Vec<i32> = tasks.iter().map(|task| task.id).collect();

        tasks_assignees = sqlx::query_as!(
            TaskAssignee,
            "
                SELECT id, user_id
                FROM task_assignee
                WHERE task_assignee.id IN (SELECT unnest($1::integer[]));
            ",
            &tasks_ids
        )
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;
    } else {
        shared_with = Vec::new();
        tasks_assignees = Vec::new();
    }

    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": {
                "owner": owner,
                "tasks": tasks,
                "lists": lists,
                "isShared": is_shared,
                "userPermissions": user_permissions,
                "sharedWith": shared_with,
                "tasksAssignees": tasks_assignees
            }
        })
        .to_string(),
    ))
}

pub async fn create_kanban(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateKanbanReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_kanban\n", "HANDLER");

    has_permission_to(&[Action::Create], Table::Kanban, user_id, &pg_pool).await?;

    let kanban_id = sqlx::query_scalar!(
        "INSERT INTO kanban (label, is_shared, owner_id) VALUES ($1, $2, $3) RETURNING id;",
        credentials.label,
        credentials.is_shared,
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

    sqlx::query!(
        "INSERT INTO user_kanban_role (kanban_id, user_id, kanban_role_id) VALUES ($1, $2, (SELECT id FROM kanban_role WHERE is_default = TRUE));",
        kanban_id,
        user_id,
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": { "kanbanId": kanban_id } }).to_string(),
    ))
}

pub async fn delete_user_from_kanban(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((kanban_id, shared_user_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_user_from_kanban\n", "HANDLER");

    has_kanban_permission_to(
        &[KanbanAction::DeleteUser],
        KanbanTable::Kanban,
        kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
            DELETE FROM shared_kanban WHERE id = $1 AND user_id = $2;
        ",
        kanban_id,
        shared_user_id,
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    sqlx::query!(
        "
            DELETE FROM user_kanban_role WHERE kanban_id = $1 AND user_id = $2;
        ",
        kanban_id,
        shared_user_id,
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    sqlx::query!(
        "
            DELETE FROM kanban_invitation WHERE kanban_id = $1 AND invitee_id = $2;
        ",
        kanban_id,
        shared_user_id,
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

pub async fn update_kanban(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(kanban_id): Path<i32>,
    Json(credentials): Json<UpdateKanbanReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - update_kanban\n", "HANDLER");

    has_permission_to(
        &[Action::UpdateOwn(user_id, kanban_id), Action::UpdateAll],
        Table::Kanban,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        UPDATE kanban SET
            label = $2
        WHERE id = $1
        ",
        kanban_id,
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

pub async fn delete_kanban(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(kanban_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_kanban\n", "HANDLER");

    has_permission_to(
        &[Action::DeleteOwn(user_id, kanban_id), Action::DeleteAll],
        Table::Kanban,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!("DELETE FROM kanban WHERE id = $1", kanban_id)
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
