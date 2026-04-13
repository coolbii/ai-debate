use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebateSide {
    Affirmative,
    Negative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebaterAgent {
    pub id: String,
    pub session_id: String,
    pub side: DebateSide,
    pub seat: String, // "first" | "second" | "third"
    pub display_name: String,
    pub persona: String,
    pub objective: String,
    pub style: String,
    pub model: String,
}
