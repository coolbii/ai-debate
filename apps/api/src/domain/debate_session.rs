use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Draft,
    Running,
    Paused,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebateSession {
    pub id: String,
    pub motion: String,
    pub definition: String,
    pub mode: String,
    pub status: SessionStatus,
    pub current_round: u32,
    pub current_phase: String,
    pub model: String,
    pub created_at: String,
    pub updated_at: String,
}
