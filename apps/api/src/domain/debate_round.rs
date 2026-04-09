#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundKind {
    Speech,
    Question,
    Answer,
    Closing,
    System,
}

#[derive(Debug, Clone)]
pub struct DebateRound {
    pub index: u32,
    pub phase: String,
    pub kind: RoundKind,
    pub speaker_id: String,
    pub target_id: Option<String>,
}
