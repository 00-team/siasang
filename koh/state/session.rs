use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::{AppErr, Config};

#[derive(Default)]
pub struct AdminSessions {
    sessions: RwLock<HashMap<u64, String>>,
}

impl AdminSessions {
    #[cfg(debug_assertions)]
    pub async fn insert(&self, id: u64, token: String) {
        self.sessions.write().await.insert(id, token);
    }

    pub async fn new_session(&self) -> (u64, String) {
        let mut ss = self.sessions.write().await;
        let id = loop {
            let id: u64 = rand::random();
            if !ss.contains_key(&id) {
                break id;
            }
        };

        let token = kaf::utils::random_string(Config::TOKEN_ABC, 64);
        ss.insert(id, token.clone());
        (id, token)
    }

    pub async fn check_session(
        &self, id: u64, token: &str,
    ) -> Result<(), AppErr> {
        let ss = self.sessions.read().await;
        let Some(st) = ss.get(&id) else { return crate::err!(BadAuth) };
        if st != token {
            return crate::err!(BadAuth);
        }

        Ok(())
    }
}
