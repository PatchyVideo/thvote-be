
use std::str::FromStr;

use serde::{Deserialize, de::DeserializeOwned};
use serde_derive::{Serialize};
use thiserror::Error;

pub static SERVICE_NAME: &'static str = "gateway";

// pub const VOTE_START: DateTime<Utc> = DateTime::from_str("2021-10-01 00:00:00GMT+8").unwrap();
// pub const VOTE_END: DateTime<Utc> = DateTime::from_str("2021-10-15 00:00:00GMT+8").unwrap();
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteTokenClaim {
	pub vote_id: Option<String>
}
