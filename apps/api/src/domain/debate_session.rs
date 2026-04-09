#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    Draft,
    Running,
    Paused,
    Finished,
}

#[derive(Debug, Clone)]
pub struct DebateSession {
    pub id: String,
    pub motion: String,
    pub definition: String,
    pub status: SessionStatus,
    pub current_round: u32,
    pub current_phase: String,
}
