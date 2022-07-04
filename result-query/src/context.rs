use std::sync::Arc;
use std::cell::Cell;

use jwt_simple::prelude::ES256kKeyPair;
use mongodb::{Collection, Database};

use crate::models::{ActivityLogEntry, Voter};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub db: Database,
    pub votes_coll: Collection<Document>,
}
