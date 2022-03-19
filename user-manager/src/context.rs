use std::sync::Arc;
use std::cell::Cell;

use jwt_simple::prelude::ES256kKeyPair;
use mongodb::{Collection, Database};

use crate::models::{ActivityLogEntry, Voter};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub vote_year: u32,
    pub vote_start: chrono::DateTime<chrono::Utc>,
    pub vote_end: chrono::DateTime<chrono::Utc>,
    pub key_pair: ES256kKeyPair,
    pub db: Database,
    pub voters_coll: Collection<Voter>,
    pub logs_coll: Collection<ActivityLogEntry>,
    pub redis_client: redis::Client
}

#[derive(Clone, Debug)]
pub struct LoginSession {
    pub thbwiki_uid: Option<String>,
    pub qq_openid: Option<String>,
    pub signup_ip: Option<String>
}

impl AppContext {
    pub fn create_login_session(&self, sess: LoginSession) -> String {
        todo!()
    }
    pub async fn get_login_session(&self, sid: &str) -> Option<LoginSession> {
        todo!()
    }
}
