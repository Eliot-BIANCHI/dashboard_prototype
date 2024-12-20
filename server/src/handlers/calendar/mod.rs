use crate::models::calendar::{
    CalendarDayQuery, CalendarMonthQuery, CalendarOverview, CalendarSchedules, CalendarWeekQuery,
    CreateCalendarReq, UpdateCalendarReq,
};
use crate::utils::authorization::{has_permission_to, Action, Table};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;
use sqlx::PgPool;

pub mod schedule;

pub async fn get_calendar_overviews(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_calendar_overviews\n", "HANDLER");

    let overviews = sqlx::query_as!(
        CalendarOverview,
        "SELECT id, label FROM calendar WHERE owner_id = $1",
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
        json!({ "success": true, "data": { "overviews": overviews } }).to_string(),
    ))
}

pub async fn get_calendar_day_schedules(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(calendar_id): Path<i32>,
    Query(params): Query<CalendarDayQuery>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_calendar_day_schedules\n", "HANDLER");

    has_permission_to(
        &[
            Action::ViewOwn(user_id, calendar_id),
            Action::ViewInvitedTo(user_id, calendar_id),
            Action::ViewAll,
        ],
        Table::Calendar,
        user_id,
        &pg_pool,
    )
    .await?;

    let rows = sqlx::query_as!(
        CalendarSchedules,
        "
        SELECT
            schedule.id,
            schedule.label,
            takes_place,
            start_time,
            end_time,
            all_day
        FROM calendar
        JOIN schedule
        ON calendar.id = schedule.calendar_id
        WHERE calendar_id = $1
        AND EXTRACT(YEAR FROM takes_place)::INT = $2
        AND EXTRACT(DOY FROM takes_place)::INT = $3;
        ",
        calendar_id,
        params.year,
        params.day
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
        json!({ "success": true, "data": { "calendarSchedules": rows } }).to_string(),
    ))
}

pub async fn get_calendar_week_schedules(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(calendar_id): Path<i32>,
    Query(params): Query<CalendarWeekQuery>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_calendar_week_schedules\n", "HANDLER");

    has_permission_to(
        &[
            Action::ViewOwn(user_id, calendar_id),
            Action::ViewInvitedTo(user_id, calendar_id),
            Action::ViewAll,
        ],
        Table::Calendar,
        user_id,
        &pg_pool,
    )
    .await?;

    let rows = sqlx::query_as!(
        CalendarSchedules,
        "
        SELECT
            schedule.id,
            schedule.label,
            takes_place,
            start_time,
            end_time,
            all_day
        FROM calendar
        JOIN schedule
        ON calendar.id = schedule.calendar_id
        WHERE calendar_id = $1
        AND EXTRACT(ISOYEAR FROM takes_place)::INT = $2
        AND EXTRACT(WEEK FROM takes_place)::INT = $3;
        ",
        calendar_id,
        params.year,
        params.week
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
        json!({ "success": true, "data": { "calendarSchedules": rows } }).to_string(),
    ))
}

pub async fn get_calendar_month_schedules(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(calendar_id): Path<i32>,
    Query(params): Query<CalendarMonthQuery>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - get_calendar_month_schedules\n", "HANDLER");

    has_permission_to(
        &[
            Action::ViewOwn(user_id, calendar_id),
            Action::ViewInvitedTo(user_id, calendar_id),
            Action::ViewAll,
        ],
        Table::Calendar,
        user_id,
        &pg_pool,
    )
    .await?;

    let rows = sqlx::query_as!(
        CalendarSchedules,
        "
        SELECT
            schedule.id,
            schedule.label,
            takes_place,
            start_time,
            end_time,
            all_day
        FROM calendar
        JOIN schedule
        ON calendar.id = schedule.calendar_id
        WHERE calendar_id = $1
        AND EXTRACT(YEAR FROM takes_place)::INT = $2
        AND EXTRACT(MONTH FROM takes_place)::INT = $3;
        ",
        calendar_id,
        params.year,
        params.month
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
        json!({ "success": true, "data": { "calendarSchedules": rows } }).to_string(),
    ))
}

pub async fn create_calendar(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Json(credentials): Json<CreateCalendarReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - create_calendar\n", "HANDLER");

    has_permission_to(&[Action::Create], Table::Calendar, user_id, &pg_pool).await?;

    let calendar_id = sqlx::query_scalar!(
        "INSERT INTO calendar (label, owner_id) VALUES ($1, $2) RETURNING id",
        credentials.label,
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
        json!({ "success": true, "data": { "calendarId": calendar_id } }).to_string(),
    ))
}

pub async fn update_calendar(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(calendar_id): Path<i32>,
    Json(credentials): Json<UpdateCalendarReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - update_calendar\n", "HANDLER");

    has_permission_to(
        &[Action::UpdateOwn(user_id, calendar_id), Action::UpdateAll],
        Table::Calendar,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!(
        "
        UPDATE calendar SET
            label = $2
        WHERE id = $1
        ",
        calendar_id,
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

pub async fn delete_calendar(
    Extension(user_id): Extension<i32>,
    State(pg_pool): State<PgPool>,
    Path(calendar_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("->> {:<12} - delete_calendar\n", "HANDLER");

    has_permission_to(
        &[Action::DeleteOwn(user_id, calendar_id), Action::DeleteAll],
        Table::Calendar,
        user_id,
        &pg_pool,
    )
    .await?;

    sqlx::query!("DELETE FROM calendar WHERE id = $1", calendar_id)
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
