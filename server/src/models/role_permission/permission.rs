use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePermissionReq {
    pub label: String,
}
