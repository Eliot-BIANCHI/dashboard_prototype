use serde::Deserialize;

use chrono::Utc;

#[derive(Deserialize)]
pub struct LogInReq {
    pub username: String,
    // pub password: String,
}

pub struct UserLogged {
    pub id: i32,
}

pub struct Session {
    pub id: String,
    pub user_id: i32,
    pub expires_at: chrono::DateTime<Utc>,
}
