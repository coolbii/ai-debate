use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundKind {
    Speech,
    Question,
    Answer,
    Closing,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebateRound {
    pub index: u32,
    pub phase: String,
    pub kind: RoundKind,
    pub speaker_id: String,
    pub target_id: Option<String>,
}
