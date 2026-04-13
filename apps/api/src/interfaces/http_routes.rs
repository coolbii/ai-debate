use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::application::{
    declare_winner_use_case::{add_judge_note, declare_winner},
    run_turn_use_case::run_next_turn,
    start_session_use_case::{create_session, start_session, CreateSessionParams},
};
use crate::state::AppState;

#[derive(Serialize)]
struct ApiError {
    error: String,
}

fn err(msg: impl ToString) -> (StatusCode, Json<ApiError>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError { error: msg.to_string() }))
}

fn not_found(msg: impl ToString) -> (StatusCode, Json<ApiError>) {
    (StatusCode::NOT_FOUND, Json(ApiError { error: msg.to_string() }))
}

// ── POST /sessions ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateSessionBody {
    pub motion: String,
    pub definition: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_model")]
    pub model: String,
}
fn default_mode() -> String { "demo".to_string() }
fn default_model() -> String { "qwen3:4b".to_string() }

async fn handle_create_session(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateSessionBody>,
) -> impl IntoResponse {
    match create_session(state, CreateSessionParams {
        motion: body.motion,
        definition: body.definition,
        mode: body.mode,
        model: body.model,
    }).await {
        Ok(session) => (StatusCode::CREATED, Json(serde_json::to_value(session).unwrap())).into_response(),
        Err(e) => err(e).into_response(),
    }
}

// ── GET /sessions/:id ────────────────────────────────────────────────────────

async fn handle_get_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let sessions = state.sessions.read().await;
    match sessions.get(&id) {
        Some(s) => Json(serde_json::to_value(s).unwrap()).into_response(),
        None => not_found("session not found").into_response(),
    }
}

// ── GET /sessions/:id/agents ─────────────────────────────────────────────────

async fn handle_get_agents(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let agents = state.agents.read().await;
    match agents.get(&id) {
        Some(a) => Json(serde_json::to_value(a).unwrap()).into_response(),
        None => not_found("agents not found").into_response(),
    }
}

// ── POST /sessions/:id/start ─────────────────────────────────────────────────

async fn handle_start(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match start_session(state, &id).await {
        Ok(session) => Json(serde_json::to_value(session).unwrap()).into_response(),
        Err(e) => err(e).into_response(),
    }
}

// ── POST /sessions/:id/pause ─────────────────────────────────────────────────

async fn handle_pause(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut sessions = state.sessions.write().await;
    match sessions.get_mut(&id) {
        Some(s) => {
            s.status = crate::domain::debate_session::SessionStatus::Paused;
            Json(serde_json::to_value(&*s).unwrap()).into_response()
        }
        None => not_found("session not found").into_response(),
    }
}

// ── POST /sessions/:id/resume ────────────────────────────────────────────────

async fn handle_resume(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut sessions = state.sessions.write().await;
    match sessions.get_mut(&id) {
        Some(s) => {
            s.status = crate::domain::debate_session::SessionStatus::Running;
            Json(serde_json::to_value(&*s).unwrap()).into_response()
        }
        None => not_found("session not found").into_response(),
    }
}

// ── POST /sessions/:id/next-turn ─────────────────────────────────────────────

async fn handle_next_turn(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match run_next_turn(state, &id).await {
        Ok(event) => Json(serde_json::to_value(event).unwrap()).into_response(),
        Err(e) => err(e).into_response(),
    }
}

// ── POST /sessions/:id/declare-winner ────────────────────────────────────────

#[derive(Deserialize)]
pub struct DeclareWinnerBody {
    pub winner: String,
    pub reasoning: String,
}

async fn handle_declare_winner(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<DeclareWinnerBody>,
) -> impl IntoResponse {
    match declare_winner(state, &id, body.winner, body.reasoning).await {
        Ok(decision) => Json(serde_json::to_value(decision).unwrap()).into_response(),
        Err(e) => err(e).into_response(),
    }
}

// ── GET /sessions/:id/events ─────────────────────────────────────────────────

async fn handle_get_events(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let events = state.events.read().await;
    let list = events.get(&id).cloned().unwrap_or_default();
    Json(serde_json::to_value(list).unwrap()).into_response()
}

// ── GET /sessions/:id/judge-decision ─────────────────────────────────────────

async fn handle_get_judge_decision(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let decisions = state.decisions.read().await;
    match decisions.get(&id) {
        Some(d) => Json(serde_json::to_value(d).unwrap()).into_response(),
        None => not_found("no decision yet").into_response(),
    }
}

// ── POST /sessions/:id/judge-notes ───────────────────────────────────────────

#[derive(Deserialize)]
pub struct JudgeNoteBody {
    pub round: u32,
    pub note: String,
}

async fn handle_judge_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<JudgeNoteBody>,
) -> impl IntoResponse {
    match add_judge_note(state, &id, body.round, body.note).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e).into_response(),
    }
}

// ── GET /health ───────────────────────────────────────────────────────────────

async fn handle_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ollama_ok = state.ollama.healthcheck().await.unwrap_or(false);
    Json(serde_json::json!({ "status": "ok", "ollama": ollama_ok }))
}

// ── Router ────────────────────────────────────────────────────────────────────

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(handle_health))
        .route("/sessions", post(handle_create_session))
        .route("/sessions/:id", get(handle_get_session))
        .route("/sessions/:id/agents", get(handle_get_agents))
        .route("/sessions/:id/start", post(handle_start))
        .route("/sessions/:id/pause", post(handle_pause))
        .route("/sessions/:id/resume", post(handle_resume))
        .route("/sessions/:id/next-turn", post(handle_next_turn))
        .route("/sessions/:id/declare-winner", post(handle_declare_winner))
        .route("/sessions/:id/events", get(handle_get_events))
        .route("/sessions/:id/judge-decision", get(handle_get_judge_decision))
        .route("/sessions/:id/judge-notes", post(handle_judge_note))
        .with_state(state)
}
