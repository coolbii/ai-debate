use super::debater_agent::DebateSide;

#[derive(Debug, Clone)]
pub struct JudgeNote {
    pub round: u32,
    pub note: String,
}

#[derive(Debug, Clone)]
pub struct JudgeDecision {
    pub session_id: String,
    pub winner: DebateSide,
    pub reasoning: String,
    pub notes_by_round: Vec<JudgeNote>,
}
