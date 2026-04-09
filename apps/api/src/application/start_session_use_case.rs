use crate::domain::debate_session::{DebateSession, SessionStatus};

pub fn start_session(mut session: DebateSession) -> DebateSession {
    session.status = SessionStatus::Running;
    session.current_round = 1;
    session.current_phase = "affirmative-first-constructive".to_string();
    session
}
