use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

use crate::domain::debate_event::DebateEvent;
use crate::domain::debate_session::DebateSession;
use crate::domain::debater_agent::DebaterAgent;
use crate::domain::judge_decision::JudgeDecision;
use crate::infrastructure::file_store::FileStore;
use crate::infrastructure::ollama_client::OllamaClient;

pub type SessionId = String;

pub struct AppState {
    pub sessions: RwLock<HashMap<SessionId, DebateSession>>,
    pub agents: RwLock<HashMap<SessionId, Vec<DebaterAgent>>>,
    pub events: RwLock<HashMap<SessionId, Vec<DebateEvent>>>,
    pub decisions: RwLock<HashMap<SessionId, JudgeDecision>>,
    /// Per-session broadcast channel — frontend WS subscribers receive serialized events
    pub event_senders: RwLock<HashMap<SessionId, broadcast::Sender<String>>>,
    pub ollama: OllamaClient,
    pub store: FileStore,
}

impl AppState {
    pub fn new(ollama_endpoint: String, default_model: String, data_dir: String) -> Arc<Self> {
        Arc::new(Self {
            sessions: RwLock::new(HashMap::new()),
            agents: RwLock::new(HashMap::new()),
            events: RwLock::new(HashMap::new()),
            decisions: RwLock::new(HashMap::new()),
            event_senders: RwLock::new(HashMap::new()),
            ollama: OllamaClient::new(ollama_endpoint, default_model),
            store: FileStore::new(data_dir),
        })
    }

    /// Get or create a broadcast sender for a session (capacity 256 messages)
    pub async fn get_or_create_sender(&self, session_id: &str) -> broadcast::Sender<String> {
        let read = self.event_senders.read().await;
        if let Some(tx) = read.get(session_id) {
            return tx.clone();
        }
        drop(read);
        let (tx, _) = broadcast::channel(256);
        let mut write = self.event_senders.write().await;
        write.insert(session_id.to_string(), tx.clone());
        tx
    }

    /// Append event to in-memory store and broadcast to WS subscribers
    pub async fn push_event(&self, event: DebateEvent) -> anyhow::Result<()> {
        let json = serde_json::to_string(&event)?;
        self.store.append_event(&event.session_id, &json).await?;

        let tx = self.get_or_create_sender(&event.session_id).await;
        // ignore send error (no receivers is fine)
        let _ = tx.send(json);

        let mut events = self.events.write().await;
        events.entry(event.session_id.clone()).or_default().push(event);
        Ok(())
    }
}
