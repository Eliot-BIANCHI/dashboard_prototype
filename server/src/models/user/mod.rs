use serde::{Deserialize, Serialize};

pub mod account;
pub mod auth;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub image_url: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOverview {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UsersOverviewsQuery {
    pub search: String,
    pub by: String,
}

#[derive(Deserialize)]
pub struct PaginatedUsersOverviewsQuery {
    pub by: String,
    pub search: Option<String>,
    pub role_id: Option<i32>,
    pub page: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreateUserReq {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UpdateUserReq {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoLoggedUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub image_url: Option<String>,
    pub role_id: i32,
    pub role_label: String,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KanbanUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub image_url: Option<String>,
    pub kanban_role_id: i32,
    pub kanban_role_label: String,
}
