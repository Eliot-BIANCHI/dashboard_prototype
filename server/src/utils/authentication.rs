use crate::models::user::auth::Session;
use chrono::{Duration, Utc};
use data_encoding::BASE32_NOPAD;
use data_encoding::HEXLOWER;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

use sqlx::PgPool;

pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 20];
    OsRng.fill_bytes(&mut bytes);

    BASE32_NOPAD.encode(&bytes).to_lowercase()
}

pub async fn create_session(
    token: &String,
    user_id: i32,
    pg_pool: &PgPool,
) -> Result<chrono::DateTime<Utc>, String> {
    let mut hasher = Sha256::new();
    hasher.update(token);
    let hash = hasher.finalize();
    let session_id = HEXLOWER.encode(&hash);

    let expires_at = Utc::now() + Duration::days(30);

    sqlx::query!(
        "INSERT INTO session (id, user_id, expires_at) VALUES ($1, $2, $3);",
        session_id,
        user_id,
        expires_at,
    )
    .execute(pg_pool)
    .await
    .map_err(|e| e.to_string())?;

    return Ok(expires_at);
}

pub async fn validate_session(token: String, pg_pool: PgPool) -> Result<Session, String> {
    let mut hasher = Sha256::new();
    hasher.update(token);
    let hash = hasher.finalize();
    let session_id = HEXLOWER.encode(&hash);

    let row = sqlx::query_as!(
        Session,
        "
            SELECT session.id, session.user_id, session.expires_at
            FROM session
            INNER JOIN app_user ON app_user.id = session.user_id
            WHERE session.id = $1;
        ",
        session_id
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(|e| e.to_string())?;

    let Some(session) = row else {
        return Err("No session for this session_id".to_owned());
    };

    if Utc::now() >= session.expires_at {
        sqlx::query!("DELETE FROM session WHERE id = $1;", session_id)
            .execute(&pg_pool)
            .await
            .map_err(|e| e.to_string())?;

        return Err("time over".to_owned());
    }

    if Utc::now() >= session.expires_at - Duration::days(15) {
        let new_expiration = Utc::now() + Duration::days(30);
        sqlx::query!(
            "UPDATE session SET expires_at = $1 WHERE id = $2;",
            new_expiration,
            session_id
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(session)
}
