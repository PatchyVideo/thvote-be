use std::sync::Arc;
use std::cell::Cell;

use bson::Document;
use mongodb::{Collection, Database};


#[derive(Clone, Debug)]
pub struct AppContext {
    pub db: Database,
    pub votes_coll: Collection<Document>,
}
