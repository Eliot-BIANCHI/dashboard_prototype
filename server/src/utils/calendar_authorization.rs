use axum::http::StatusCode;
use serde_json::json;

use sqlx::PgPool;

pub enum CalendarAction {
    // InviteUser,
    // DeleteUser,
    // KickUser,
    Create,
    UpdateOwn(i32, i32),
    DeleteOwn(i32, i32),
    UpdateAll,
    DeleteAll,
}

pub enum CalendarTable {
    // Calendar,
    Schedule,
}

enum As {
    Owner(i32, i32),
}

pub async fn has_calendar_permission_to(
    actions: &[CalendarAction],
    table: CalendarTable,
    calendar_id: i32,
    user_id: i32,
    pg_pool: &PgPool,
) -> Result<(), (StatusCode, String)> {
    let has_access = sqlx::query_scalar!(
        "
            SELECT EXISTS (
                SELECT 1
                FROM calendar
                WHERE (calendar.id = $1 AND calendar.owner_id = $2)
                OR calendar.id IN (
                    SELECT shared_calendar.id
                    FROM shared_calendar
                    WHERE shared_calendar.id = $1 AND shared_calendar.user_id = $2
                )
            );
        ",
        calendar_id,
        user_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    if !has_access.unwrap_or(false) {
        return Err((
            StatusCode::UNAUTHORIZED,
            json!({ "success": false, "message": format!("User doesn't have the rights to access this section") })
                .to_string(),
        ));
    }

    for action in actions {
        let (action_name, action_as) = get_action_name_and_as(action);

        let table_name = get_table_name(&table);

        let permission = format!("{}:{}", table_name, action_name);

        let has_permission = sqlx::query_scalar!(
            "
                SELECT EXISTS (
                    SELECT 1
                    FROM user_calendar_role
                    INNER JOIN calendar_role_permission ON calendar_role_permission.calendar_role_id = user_calendar_role.calendar_role_id
                    INNER JOIN permission ON permission.id = calendar_role_permission.permission_id
                    WHERE user_calendar_role.user_id = $1 AND user_calendar_role.calendar_id = $2
                    AND permission.label = $3
                );
            ",
            user_id,
            calendar_id,
            permission
        )
        .fetch_one(pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

        if !has_permission.unwrap_or(false) {
            continue;
        }

        let Some(r#as) = action_as else { return Ok(()) };

        match r#as {
            As::Owner(owner_id, table_id) => {
                if is_owner_of(table_name, table_id, owner_id, pg_pool).await? {
                    return Ok(());
                }
            }
        }
    }

    Err((
        StatusCode::FORBIDDEN,
        json!({ "success": false, "message": format!("User doesn't have the rights to make this action") })
            .to_string(),
    ))
}

async fn is_owner_of(
    table_name: &str,
    table_id: i32,
    user_id: i32,
    pg_pool: &PgPool,
) -> Result<bool, (StatusCode, String)> {
    let query = format!("SELECT owner_id FROM {} WHERE id = $1", table_name);

    let row: Option<i32> = sqlx::query_scalar(&query)
        .bind(table_id)
        .fetch_optional(pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    let Some(owner_id) = row else {
        return Err((
            StatusCode::NOT_FOUND,
            json!({ "success": false, "message": format!("No {} found for this ID", table_name) })
                .to_string(),
        ));
    };

    Ok(owner_id == user_id)
}

fn get_action_name_and_as(action: &CalendarAction) -> (&str, Option<As>) {
    match action {
        // CalendarAction::KickUser => ("kick-user", None),
        // CalendarAction::InviteUser => ("invite-user", None),
        // CalendarAction::DeleteUser => ("delete-user", None),
        CalendarAction::Create => ("create", None),
        CalendarAction::UpdateOwn(owner_id, table_id) => {
            ("update-own", Some(As::Owner(*owner_id, *table_id)))
        }
        CalendarAction::DeleteOwn(owner_id, table_id) => {
            ("delete-own", Some(As::Owner(*owner_id, *table_id)))
        }
        CalendarAction::UpdateAll => ("update-all", None),
        CalendarAction::DeleteAll => ("delete-all", None),
    }
}

fn get_table_name(table: &CalendarTable) -> &str {
    match table {
        // CalendarTable::Calendar => "calendar",
        CalendarTable::Schedule => "schedule",
    }
}
