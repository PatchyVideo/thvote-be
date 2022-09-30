use std::sync::Arc;
use std::cell::Cell;

use bson::Document;
use mongodb::{Collection, Database};

use crate::models::{CachedRankingEntry, CachedRankingGlobal, CachedCPRankingEntry, PartialVoteItemEntry, GlobalStats, CompletionRate, CachedQuestionEntry, CachedCovote};


#[derive(Clone, Debug)]
pub struct AppContext {
    pub db: Database,
	pub lock: redlock::RedLock,
    pub votes_coll: Collection<Document>,
    pub chars_entry_cache_coll: Collection<CachedRankingEntry>,
    pub chars_global_cache_coll: Collection<CachedRankingGlobal>,
    pub musics_entry_cache_coll: Collection<CachedRankingEntry>,
    pub musics_global_cache_coll: Collection<CachedRankingGlobal>,
    pub cps_entry_cache_coll: Collection<CachedCPRankingEntry>,
    pub cps_global_cache_coll: Collection<CachedRankingGlobal>,
    pub all_chars: Collection<PartialVoteItemEntry>,
    pub all_musics: Collection<PartialVoteItemEntry>,
    pub global_stats: Collection<GlobalStats>,
    pub completion_rates: Collection<CompletionRate>,
    pub paper_result: Collection<CachedQuestionEntry>,
    pub covote_musics: Collection<CachedCovote>,
    pub covote_chars: Collection<CachedCovote>,
}
