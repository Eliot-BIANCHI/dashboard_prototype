// use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KanbanInvitation {
    pub id: i32,
    pub kanban_label: String,
    pub inviter_first_name: String,
    pub inviter_last_name: String,
}

pub struct AcceptKanbanInvitation {
    pub kanban_id: i32,
    pub kanban_role_id: i32,
}

#[derive(Deserialize)]
pub struct CreateKanbanInvitationReq {
    pub kanban_id: i32,
    pub invitee_id: i32,
    pub kanban_role_id: i32,
}
