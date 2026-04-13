use std::sync::Arc;
use anyhow::Result;
use chrono::Utc;

use crate::domain::judge_decision::JudgeDecision;
use crate::state::AppState;

pub async fn declare_winner(
    state: Arc<AppState>,
    session_id: &str,
    winner: String,
    reasoning: String,
) -> Result<JudgeDecision> {
    let decision = JudgeDecision {
        session_id: session_id.to_string(),
        winner,
        reasoning,
        notes_by_round: {
            // Collect any judge_note events as notes_by_round
            let events = state.events.read().await;
            events.get(session_id).cloned().unwrap_or_default()
                .into_iter()
                .filter(|e| matches!(e.kind, crate::domain::debate_event::EventKind::JudgeNote))
                .map(|e| crate::domain::judge_decision::JudgeNote {
                    round: e.round,
                    note: e.content,
                })
                .collect()
        },
        decided_at: Utc::now().to_rfc3339(),
    };

    let json = serde_json::to_string_pretty(&decision)?;
    state.store.write_judge_decision(session_id, &json).await?;
    state.decisions.write().await.insert(session_id.to_string(), decision.clone());

    Ok(decision)
}

pub async fn add_judge_note(
    state: Arc<AppState>,
    session_id: &str,
    round: u32,
    note: String,
) -> Result<()> {
    use crate::domain::debate_event::{DebateEvent, EventKind};
    use uuid::Uuid;

    let event = DebateEvent {
        id: Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        round,
        phase: "judge-note".to_string(),
        speaker_id: "judge".to_string(),
        target_id: None,
        kind: EventKind::JudgeNote,
        content: note,
        meta: None,
        started_at: Utc::now().to_rfc3339(),
        ended_at: None,
    };
    state.push_event(event).await
}
