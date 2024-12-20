use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRoleReq {
    pub label: String,
}
