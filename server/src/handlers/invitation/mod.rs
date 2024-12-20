use crate::models::invitation::KanbanInvitation;
use axum::{extract::State, http::StatusCode, Extension};
use serde_json::json;

use sqlx::PgPool;

pub mod kanban;

pub async fn get_pending_invitations(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_pending_invitations\n", "HANDLER");

    let kanbans_invitations = sqlx::query_as!(
        KanbanInvitation,
        "
            SELECT
                kanban_invitation.id, kanban.label AS kanban_label,
                inviter.first_name AS inviter_first_name, inviter.last_name AS inviter_last_name
            FROM kanban_invitation
            JOIN kanban ON kanban.id = kanban_invitation.kanban_id
            JOIN app_user AS inviter ON inviter.id = kanban_invitation.inviter_id
            WHERE invitee_id = $1 AND status = 'pending';
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
        json!({ "success": true, "data": { "kanbansInvitations": kanbans_invitations } })
            .to_string(),
    ))
}

pub async fn get_received_invitations(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_received_invitations\n", "HANDLER");

    let kanbans_invitations = sqlx::query_as!(
        KanbanInvitation,
        "
            SELECT
                kanban_invitation.id, kanban.label AS kanban_label,
                inviter.first_name AS inviter_first_name, inviter.last_name AS inviter_last_name
            FROM kanban_invitation
            JOIN kanban ON kanban.id = kanban_invitation.kanban_id
            JOIN app_user AS inviter ON inviter.id = kanban_invitation.inviter_id
            WHERE invitee_id = $1;
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
        json!({ "success": true, "data": { "kanbansInvitations": kanbans_invitations } })
            .to_string(),
    ))
}

pub async fn get_sent_invitations(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_sent_invitations\n", "HANDLER");

    let kanbans_invitations = sqlx::query_as!(
        KanbanInvitation,
        "
            SELECT
                kanban_invitation.id, kanban.label AS kanban_label,
                inviter.first_name AS inviter_first_name, inviter.last_name AS inviter_last_name
            FROM kanban_invitation
            JOIN kanban ON kanban.id = kanban_invitation.kanban_id
            JOIN app_user AS inviter ON inviter.id = kanban_invitation.inviter_id
            WHERE invitee_id = $1 AND status = 'pending';
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
        json!({ "success": true, "data": { "kanbansInvitations": kanbans_invitations } })
            .to_string(),
    ))
}
