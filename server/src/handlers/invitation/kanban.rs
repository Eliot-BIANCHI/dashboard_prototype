use crate::models::invitation::{AcceptKanbanInvitation, CreateKanbanInvitationReq};
use crate::utils::kanban_authorization::{has_kanban_permission_to, KanbanAction, KanbanTable};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;

use sqlx::PgPool;

pub async fn create_kanban_invitation(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateKanbanInvitationReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_kanban_invitation\n", "HANDLER");

    has_kanban_permission_to(
        &[KanbanAction::InviteUser],
        KanbanTable::Kanban,
        credentials.kanban_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
            INSERT INTO kanban_invitation (kanban_id, inviter_id, invitee_id, kanban_role_id)
            VALUES ($1, $2, $3, $4);
        ",
        credentials.kanban_id,
        user_id,
        credentials.invitee_id,
        credentials.kanban_role_id,
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

pub async fn accept_kanban_invitation(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(invitation_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - accept_kanban_invitation\n", "HANDLER");

    let invitation = sqlx::query_as!(
        AcceptKanbanInvitation,
        "
            SELECT kanban_id, kanban_role_id
            FROM kanban_invitation
            WHERE id = $1 AND invitee_id = $2 AND status = 'pending';
        ",
        invitation_id,
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
        "UPDATE kanban_invitation SET status = 'accepted' WHERE id = $1",
        invitation_id
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
        "INSERT INTO shared_kanban(id, user_id) VALUES ($1, $2);",
        invitation.kanban_id,
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

    sqlx::query!(
        "INSERT INTO user_kanban_role (kanban_id, kanban_role_id, user_id) VALUES ($1, $2, $3);",
        invitation.kanban_id,
        invitation.kanban_role_id,
        user_id
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

pub async fn reject_kanban_invitation(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(invitation_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "UPDATE kanban_invitation SET status = 'rejected' WHERE id = $1 AND invitee_id = $2",
        invitation_id,
        user_id
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
