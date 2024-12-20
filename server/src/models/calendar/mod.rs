use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

pub mod schedule;

#[derive(Serialize, Deserialize)]
pub struct CalendarOverview {
    pub id: i32,
    pub label: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarSchedules {
    pub id: i32,
    pub label: String,
    pub takes_place: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub all_day: bool,
}

#[derive(Deserialize)]
pub struct CreateCalendarReq {
    pub label: String,
}

#[derive(Deserialize)]
pub struct CalendarDayQuery {
    pub year: i32,
    pub day: i32,
}

#[derive(Deserialize)]
pub struct CalendarWeekQuery {
    pub year: i32,
    pub week: i32,
}

#[derive(Deserialize)]
pub struct CalendarMonthQuery {
    pub year: i32,
    pub month: i32,
}

#[derive(Deserialize)]
pub struct UpdateCalendarReq {
    pub label: String,
}
