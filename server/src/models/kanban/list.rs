use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub id: i32,
    pub label: String,
    pub owner_id: i32,
}

#[derive(Deserialize)]
pub struct CreateListReq {
    pub label: String,
}

#[derive(Deserialize)]
pub struct UpdateListReq {
    pub label: String,
}
