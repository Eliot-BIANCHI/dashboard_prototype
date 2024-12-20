use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateAccountReq {
    pub first_name: String,
    pub last_name: String,
}
