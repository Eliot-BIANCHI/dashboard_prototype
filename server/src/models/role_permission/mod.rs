use serde::{Deserialize, Serialize};

pub mod permission;
pub mod role;

#[derive(Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KanbanRole {
    pub id: i32,
    pub label: String,
    pub is_default: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarRole {
    pub id: i32,
    pub label: String,
    pub is_default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub id: i32,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarRolePermission {
    pub calendar_role_id: i32,
    pub permission_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KanbanRolePermission {
    pub kanban_role_id: i32,
    pub permission_id: i32,
}
