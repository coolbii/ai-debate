use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Speech,
    Question,
    Answer,
    System,
    JudgeNote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebateEvent {
    pub id: String,
    pub session_id: String,
    pub round: u32,
    pub phase: String,
    pub speaker_id: String,
    pub target_id: Option<String>,
    pub kind: EventKind,
    pub content: String,
    pub meta: Option<serde_json::Value>,
    pub started_at: String,
    pub ended_at: Option<String>,
}
