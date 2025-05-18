use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: i64,
    pub nickname: String,
    pub connected_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub is_typing: bool,
    pub current_conversation: Option<String>,
    pub current_group: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<i64, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_session(&self, user_id: i64, nickname: String) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(
            user_id,
            Session {
                user_id,
                nickname,
                connected_at: Utc::now(),
                last_active: Utc::now(),
                is_typing: false,
                current_conversation: None,
                current_group: None,
            },
        );
    }

    pub async fn remove_session(&self, user_id: i64) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&user_id);
    }

    pub async fn update_session_activity(&self, user_id: i64) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&user_id) {
            session.last_active = Utc::now();
        }
    }

    pub async fn set_typing_status(&self, user_id: i64, is_typing: bool) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&user_id) {
            session.is_typing = is_typing;
        }
    }

    pub async fn set_current_conversation(&self, user_id: i64, conversation_id: Option<String>) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&user_id) {
            session.current_conversation = conversation_id;
        }
    }

    pub async fn set_current_group(&self, user_id: i64, group_id: Option<String>) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&user_id) {
            session.current_group = group_id;
        }
    }

    pub async fn get_session(&self, user_id: i64) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(&user_id).cloned()
    }

    pub async fn get_all_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }

    pub async fn get_active_sessions(&self, timeout_seconds: i64) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        let now = Utc::now();
        sessions
            .values()
            .filter(|session| (now - session.last_active).num_seconds() < timeout_seconds)
            .cloned()
            .collect()
    }
}
