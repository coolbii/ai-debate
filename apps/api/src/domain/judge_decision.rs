use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JudgeNote {
    pub round: u32,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JudgeDecision {
    pub session_id: String,
    pub winner: String, // "affirmative" | "negative"
    pub reasoning: String,
    pub notes_by_round: Vec<JudgeNote>,
    pub decided_at: String,
}
