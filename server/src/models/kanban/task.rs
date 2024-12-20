use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i32,
    pub label: String,
    pub completion_id: i32,
    pub list_id: i32,
    pub owner_id: i32,
    pub created_at: NaiveDate,
    pub priority: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskAssignee {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct CreateTaskReq {
    pub label: String,
    pub completion_id: i32,
    pub priority: bool,
}

#[derive(Deserialize)]
pub struct UpdateTaskReq {
    pub label: String,
    pub completion_id: i32,
    pub priority: bool,
}

#[derive(Deserialize)]
pub struct PatchTaskListReq {
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct PatchTaskCompletionReq {
    pub completion_id: i32,
}

#[derive(Deserialize)]
pub struct AssignUser {
    pub user_id: i32,
}
