use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateScheduleReq {
    pub label: String,
    pub takes_place: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub all_day: bool,
    pub calendar_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateScheduleReq {
    pub label: String,
    pub takes_place: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub all_day: bool,
}
