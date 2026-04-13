use std::sync::Arc;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::domain::debate_event::{DebateEvent, EventKind};
use crate::domain::debate_session::{DebateSession, SessionStatus};
use crate::domain::debater_agent::{DebaterAgent, DebateSide};
use crate::state::AppState;

pub struct CreateSessionParams {
    pub motion: String,
    pub definition: String,
    pub mode: String,   // "formal" | "demo"
    pub model: String,
}

fn default_agents(session_id: &str, model: &str) -> Vec<DebaterAgent> {
    let seats = ["first", "second", "third"];
    let mut agents = Vec::with_capacity(6);

    for seat in &seats {
        agents.push(DebaterAgent {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            side: DebateSide::Affirmative,
            seat: seat.to_string(),
            display_name: format!("Affirmative {}", capitalize(seat)),
            persona: format!("A confident and logical debater arguing in favour of the motion ({} seat)", seat),
            objective: "Convince the judge that the motion is true with sound reasoning and evidence.".to_string(),
            style: "Clear, structured, persuasive".to_string(),
            model: model.to_string(),
        });
    }
    for seat in &seats {
        agents.push(DebaterAgent {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            side: DebateSide::Negative,
            seat: seat.to_string(),
            display_name: format!("Negative {}", capitalize(seat)),
            persona: format!("A sharp and analytical debater opposing the motion ({} seat)", seat),
            objective: "Convince the judge that the motion is false or should not be accepted.".to_string(),
            style: "Critical, incisive, counter-argumentative".to_string(),
            model: model.to_string(),
        });
    }
    agents
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub async fn create_session(state: Arc<AppState>, params: CreateSessionParams) -> Result<DebateSession> {
    let now = Utc::now().to_rfc3339();
    let session_id = Uuid::new_v4().to_string();

    let session = DebateSession {
        id: session_id.clone(),
        motion: params.motion,
        definition: params.definition,
        mode: params.mode,
        status: SessionStatus::Draft,
        current_round: 0,
        current_phase: "".to_string(),
        model: params.model.clone(),
        created_at: now.clone(),
        updated_at: now,
    };

    let agents = default_agents(&session_id, &params.model);

    // Persist
    let session_json = serde_json::to_string_pretty(&session)?;
    state.store.write_session(&session_id, &session_json).await?;

    state.sessions.write().await.insert(session_id.clone(), session.clone());
    state.agents.write().await.insert(session_id.clone(), agents);
    state.events.write().await.insert(session_id.clone(), Vec::new());

    Ok(session)
}

pub async fn start_session(state: Arc<AppState>, session_id: &str) -> Result<DebateSession> {
    let mut sessions = state.sessions.write().await;
    let session = sessions.get_mut(session_id).ok_or_else(|| anyhow::anyhow!("session not found"))?;

    session.status = SessionStatus::Running;
    session.current_round = 1;
    session.current_phase = "affirmative-first-constructive".to_string();
    session.updated_at = Utc::now().to_rfc3339();

    let session_clone = session.clone();
    drop(sessions);

    // Persist
    let session_json = serde_json::to_string_pretty(&session_clone)?;
    state.store.write_session(session_id, &session_json).await?;

    // Emit system event
    let event = DebateEvent {
        id: Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        round: 0,
        phase: "session-start".to_string(),
        speaker_id: "system".to_string(),
        target_id: None,
        kind: EventKind::System,
        content: "Debate session started.".to_string(),
        meta: None,
        started_at: Utc::now().to_rfc3339(),
        ended_at: None,
    };
    state.push_event(event).await?;

    Ok(session_clone)
}
