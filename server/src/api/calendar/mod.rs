use crate::handlers::calendar::{
    create_calendar, delete_calendar, get_calendar_day_schedules, get_calendar_month_schedules,
    get_calendar_overviews, get_calendar_week_schedules, update_calendar,
};
use axum::{routing::get, routing::put, Router};
use sqlx::PgPool;

pub mod schedule;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_calendar_overviews).post(create_calendar))
        .route("/:calendar_id/days", get(get_calendar_day_schedules))
        .route("/:calendar_id/weeks", get(get_calendar_week_schedules))
        .route("/:calendar_id/months", get(get_calendar_month_schedules))
        .route(
            "/:calendar_id",
            put(update_calendar).delete(delete_calendar),
        )
}
