use serde::{Deserialize, Serialize};

pub mod list;
pub mod task;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    pub id: i32,
    pub label: String,
    pub is_shared: bool,
}

#[derive(Deserialize)]
pub struct CreateKanbanReq {
    pub label: String,
    pub is_shared: bool,
}

#[derive(Deserialize)]
pub struct UpdateKanbanReq {
    pub label: String,
}
