use anyhow::Result;
use std::path::PathBuf;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone)]
pub struct FileStore {
    pub root_dir: String,
}

impl FileStore {
    pub fn new(root_dir: String) -> Self {
        Self { root_dir }
    }

    fn session_dir(&self, session_id: &str) -> PathBuf {
        PathBuf::from(&self.root_dir).join(session_id)
    }

    pub async fn ensure_session_dir(&self, session_id: &str) -> Result<()> {
        fs::create_dir_all(self.session_dir(session_id)).await?;
        Ok(())
    }

    /// Append a JSON line to events.jsonl
    pub async fn append_event(&self, session_id: &str, json_line: &str) -> Result<()> {
        self.ensure_session_dir(session_id).await?;
        let path = self.session_dir(session_id).join("events.jsonl");
        let mut file = OpenOptions::new().create(true).append(true).open(path).await?;
        file.write_all(format!("{}\n", json_line).as_bytes()).await?;
        Ok(())
    }

    /// Write / overwrite session.json
    pub async fn write_session(&self, session_id: &str, json: &str) -> Result<()> {
        self.ensure_session_dir(session_id).await?;
        let path = self.session_dir(session_id).join("session.json");
        fs::write(path, json).await?;
        Ok(())
    }

    /// Write / overwrite judge-decision.json
    pub async fn write_judge_decision(&self, session_id: &str, json: &str) -> Result<()> {
        self.ensure_session_dir(session_id).await?;
        let path = self.session_dir(session_id).join("judge-decision.json");
        fs::write(path, json).await?;
        Ok(())
    }

    /// Build transcript.md from events (returns markdown string)
    pub async fn read_events_jsonl(&self, session_id: &str) -> Result<String> {
        let path = self.session_dir(session_id).join("events.jsonl");
        match fs::read_to_string(path).await {
            Ok(s) => Ok(s),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
            Err(e) => Err(e.into()),
        }
    }
}
