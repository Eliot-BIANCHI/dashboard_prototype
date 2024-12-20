use crate::models::calendar::schedule::{CreateScheduleReq, UpdateScheduleReq};
use crate::utils::calendar_authorization::{
    has_calendar_permission_to, CalendarAction, CalendarTable,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;
use sqlx::PgPool;

pub async fn create_schedule(
    Extension(user_id): Extension<i32>,
    Path(calendar_id): Path<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateScheduleReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_schedule\n", "HANDLER");

    has_calendar_permission_to(
        &[CalendarAction::Create],
        CalendarTable::Schedule,
        calendar_id,
        user_id,
        &pg_pool,
    )
    .await?;

    let schedule_id = sqlx::query_scalar!(
        "
            INSERT INTO schedule (label, takes_place, start_time, end_time, all_day, calendar_id, owner_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
        ",
        credentials.label,
        credentials.takes_place,
        credentials.start_time,
        credentials.end_time,
        credentials.all_day,
        credentials.calendar_id,
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
        json!({ "success": true, "data": { "scheduleId": schedule_id } }).to_string(),
    ))
}

pub async fn update_schedule(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((calendar_id, schedule_id)): Path<(i32, i32)>,
    Json(credentials): Json<UpdateScheduleReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - update_schedule\n", "HANDLER");

    has_calendar_permission_to(
        &[
            CalendarAction::UpdateOwn(user_id, schedule_id),
            CalendarAction::UpdateAll,
        ],
        CalendarTable::Schedule,
        calendar_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        UPDATE schedule SET
            label = $2,
            takes_place = $3,
            start_time = $4,
            end_time = $5,
            all_day = $6
        WHERE id = $1
        ",
        schedule_id,
        credentials.label,
        credentials.takes_place,
        credentials.start_time,
        credentials.end_time,
        credentials.all_day,
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

pub async fn delete_schedule(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path((calendar_id, schedule_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_schedule\n", "HANDLER");

    has_calendar_permission_to(
        &[
            CalendarAction::DeleteOwn(user_id, schedule_id),
            CalendarAction::DeleteAll,
        ],
        CalendarTable::Schedule,
        calendar_id,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!("DELETE FROM schedule WHERE id = $1", schedule_id)
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
