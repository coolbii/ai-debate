use std::sync::Arc;
use anyhow::{anyhow, Result};
use chrono::Utc;
use uuid::Uuid;

use crate::domain::debate_event::{DebateEvent, EventKind};
use crate::domain::debate_session::SessionStatus;
use crate::domain::debater_agent::DebateSide;
use crate::domain::oregon_protocol::{get_turn, TurnKind};
use crate::state::AppState;

pub async fn run_next_turn(state: Arc<AppState>, session_id: &str) -> Result<DebateEvent> {
    // Read session state
    let (current_round, motion, definition, model, mode, status) = {
        let sessions = state.sessions.read().await;
        let s = sessions.get(session_id).ok_or_else(|| anyhow!("session not found"))?;
        if s.status != SessionStatus::Running {
            return Err(anyhow!("session is not running (status: {:?})", s.status));
        }
        (s.current_round, s.motion.clone(), s.definition.clone(), s.model.clone(), s.mode.clone(), s.status.clone())
    };
    let _ = status; // suppress warning

    let turn = get_turn(current_round).ok_or_else(|| anyhow!("no turn plan for round {}", current_round))?;

    // Find the speaker agent
    let agents = state.agents.read().await;
    let session_agents = agents.get(session_id).ok_or_else(|| anyhow!("agents not found"))?;

    let (speaker, target) = match turn.kind {
        TurnKind::Closing => {
            // Closing: no specific speaker — we synthesise from both sides
            (None, None)
        }
        _ => {
            let spk_side = turn.speaker_side.unwrap();
            let spk_seat = turn.speaker_seat.unwrap();
            let speaker = session_agents.iter().find(|a| {
                let side_str = match a.side { DebateSide::Affirmative => "affirmative", DebateSide::Negative => "negative" };
                side_str == spk_side && a.seat == spk_seat
            }).ok_or_else(|| anyhow!("speaker not found: {} {}", spk_side, spk_seat))?;

            let tgt = if let (Some(ts), Some(tseat)) = (turn.target_side, turn.target_seat) {
                session_agents.iter().find(|a| {
                    let side_str = match a.side { DebateSide::Affirmative => "affirmative", DebateSide::Negative => "negative" };
                    side_str == ts && a.seat == tseat
                })
            } else {
                None
            };
            (Some(speaker), tgt)
        }
    };

    // Build event context: recent transcript (last 5 events)
    let recent_transcript = {
        let events = state.events.read().await;
        let evs = events.get(session_id).cloned().unwrap_or_default();
        drop(events);
        let recent: Vec<String> = evs.iter().rev().take(5).rev()
            .map(|e| format!("[{}] {}: {}", e.phase, e.speaker_id, e.content))
            .collect();
        recent.join("\n")
    };
    drop(agents);

    let started_at = Utc::now().to_rfc3339();

    // For closing, generate both sides closing statements
    if turn.kind == TurnKind::Closing {
        return run_closing_turn(state, session_id, &motion, &definition, &model, &mode, &recent_transcript, turn.phase, started_at).await;
    }

    let speaker = speaker.unwrap();
    let speaker_id = speaker.id.clone();
    let speaker_name = speaker.display_name.clone();
    let speaker_persona = speaker.persona.clone();
    let speaker_objective = speaker.objective.clone();
    let speaker_style = speaker.style.clone();
    let target_id = target.map(|t| t.id.clone());
    let target_name = target.map(|t| t.display_name.clone());

    let (kind, task_instruction) = match turn.kind {
        TurnKind::Speech => (
            EventKind::Speech,
            format!(
                "Deliver your constructive speech. Present your strongest arguments for your side.\n\
                 Be persuasive and structured. Do NOT introduce arguments that contradict your side's position.",
            ),
        ),
        TurnKind::Question => (
            EventKind::Question,
            format!(
                "You are cross-examining {}. Ask sharp, probing questions that expose weaknesses in their argument.\n\
                 Ask 2–3 concise questions. Do not answer — only question.",
                target_name.as_deref().unwrap_or("the opposing debater"),
            ),
        ),
        TurnKind::Closing => unreachable!(),
    };

    let prompt = build_prompt(
        &motion, &definition,
        &speaker_name, &speaker_persona, &speaker_objective, &speaker_style,
        turn.phase, &recent_transcript, &task_instruction,
        &mode,
    );

    let output = state.ollama.generate(&model, &prompt).await
        .map_err(|e| anyhow!("Ollama error on round {}: {}", current_round, e))?;

    let ended_at = Utc::now().to_rfc3339();

    let meta = serde_json::json!({
        "key_claims": output.key_claims,
        "attack_targets": output.attack_targets,
        "defense_targets": output.defense_targets,
        "round_summary": output.round_summary,
    });

    let event = DebateEvent {
        id: Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        round: current_round,
        phase: turn.phase.to_string(),
        speaker_id: speaker_id.clone(),
        target_id,
        kind,
        content: output.public_speech,
        meta: Some(meta),
        started_at,
        ended_at: Some(ended_at),
    };

    state.push_event(event.clone()).await?;

    // Advance round
    advance_round(state, session_id, current_round).await?;

    Ok(event)
}

async fn run_closing_turn(
    state: Arc<AppState>,
    session_id: &str,
    motion: &str,
    definition: &str,
    model: &str,
    mode: &str,
    recent_transcript: &str,
    phase: &str,
    started_at: String,
) -> Result<DebateEvent> {
    let closing_prompt = format!(
        "Motion: \"{motion}\"\nDefinition: {definition}\n\n\
         Recent debate:\n{recent_transcript}\n\n\
         You are the SYSTEM ORCHESTRATOR. Write a closing summary of the debate in JSON format.\n\
         Summarise the key arguments made by both sides and the state of the debate.\n\
         Mode: {mode}. Be concise.",
    );

    let output = state.ollama.generate(model, &closing_prompt).await
        .map_err(|e| anyhow!("Ollama closing error: {}", e))?;

    let ended_at = Utc::now().to_rfc3339();

    let event = DebateEvent {
        id: Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        round: 13,
        phase: phase.to_string(),
        speaker_id: "system".to_string(),
        target_id: None,
        kind: EventKind::System,
        content: output.public_speech,
        meta: Some(serde_json::json!({ "round_summary": output.round_summary })),
        started_at,
        ended_at: Some(ended_at),
    };

    state.push_event(event.clone()).await?;
    advance_round(state, session_id, 13).await?;

    Ok(event)
}

async fn advance_round(state: Arc<AppState>, session_id: &str, current_round: u32) -> Result<()> {
    let mut sessions = state.sessions.write().await;
    let session = sessions.get_mut(session_id).ok_or_else(|| anyhow!("session not found"))?;

    let next_round = current_round + 1;
    if next_round > 13 {
        session.status = SessionStatus::Finished;
        session.current_round = 13;
        session.current_phase = "finished".to_string();
    } else {
        session.current_round = next_round;
        if let Some(turn) = get_turn(next_round) {
            session.current_phase = turn.phase.to_string();
        }
    }
    session.updated_at = Utc::now().to_rfc3339();

    let session_json = serde_json::to_string_pretty(&*session)?;
    let sid = session_id.to_string();
    drop(sessions);

    state.store.write_session(&sid, &session_json).await?;
    Ok(())
}

fn build_prompt(
    motion: &str,
    definition: &str,
    speaker_name: &str,
    persona: &str,
    objective: &str,
    style: &str,
    phase: &str,
    recent_transcript: &str,
    task: &str,
    mode: &str,
) -> String {
    let word_limit = if mode == "demo" { "Keep your response under 150 words." } else { "Keep your response under 300 words." };

    format!(
        "You are {speaker_name}.\n\
         Persona: {persona}\n\
         Objective: {objective}\n\
         Style: {style}\n\n\
         MOTION: \"{motion}\"\n\
         DEFINITION: {definition}\n\n\
         CURRENT PHASE: {phase}\n\n\
         RECENT DEBATE TRANSCRIPT:\n{recent_transcript}\n\n\
         YOUR TASK: {task}\n\n\
         {word_limit}\n\n\
         Respond in JSON with these fields:\n\
         - public_speech: your spoken words\n\
         - key_claims: array of 1-3 main claims you are making\n\
         - attack_targets: claims from the other side you are challenging\n\
         - defense_targets: your own claims you are defending\n\
         - round_summary: one sentence summary of what you did this turn"
    )
}
