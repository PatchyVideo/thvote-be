use std::sync::Arc;
use std::cell::Cell;

use bson::Document;
use mongodb::{Collection, Database};

use crate::models::{CachedRankingEntry, CachedRankingGlobal};


#[derive(Clone, Debug)]
pub struct AppContext {
    pub db: Database,
    pub votes_coll: Collection<Document>,
    pub chars_entry_cache_coll: Collection<CachedRankingEntry>,
    pub chars_global_cache_coll: Collection<CachedRankingGlobal>,
}
