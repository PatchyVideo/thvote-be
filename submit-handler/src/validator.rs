
use std::collections::HashSet;

use actix_web::Error;
use bson::doc;
use futures_util::TryStreamExt;
use mongodb::Collection;
use pvrustlib::ServiceError;

use crate::{models::{self, *}, common::SERVICE_NAME};

#[derive(Debug, Clone)]
pub struct SubmitValidatorV1 {
	pub all_characters: HashSet<String>,
	pub all_music: HashSet<String>
}

impl SubmitValidatorV1 {
	pub async fn new() -> Self {
		Self {
			all_characters: HashSet::new(),
			all_music: HashSet::new()
		}
	}
	pub async fn validate_character(&self, mut data: models::CharacterSubmitRest, coll: &Collection<CharacterSubmitRest>) -> Result<models::CharacterSubmitRest, ServiceError> {
		// step 2: retrieve and check if user attempts are allowed

		// first we lock submit for this vote_id
		let query = doc! {
			"meta.vote_id": data.meta.vote_id.clone()
		};
		let mut found_attempts = match coll.find(query, None).await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); }
		};
		// step 3: check ranks are unique from 1 to 6 and only one 本命
		let mut chset: HashSet<String> = HashSet::new();
		let mut first_set = false;
		if data.characters.len() < 1 || data.characters.len() > 8 {
			return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("数量{}不在范围内[1,8]", data.characters.len())));
		}
		for c in data.characters.iter() {
			if c.reason.as_ref().map_or(0, |f| f.len()) > 4096 {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", "理由过长".into()));
			}
			if c.first.unwrap_or_default() {
				if first_set {
					return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", "多个本命".into()));
				}
				first_set = true;
			}
			if chset.contains(&c.name) {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("{}已存在", c.name)));
			}
			chset.insert(c.name.clone());
		}
		// step 4: check all names are correct
		// step 5: return
		Ok(data)
	}
	pub async fn validate_music(&self, mut data: models::MusicSubmitRest, coll: &Collection<MusicSubmitRest>) -> Result<models::MusicSubmitRest, ServiceError> {
		let query = doc! {
			"meta.vote_id": data.meta.vote_id.clone()
		};
		// step 3: check ranks are unique from 1 to 6 and only one 本命
		let mut chset: HashSet<String> = HashSet::new();
		let mut first_set = false;
		if data.music.len() < 1 || data.music.len() > 12 {
			return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("数量{}不在范围内[1,12]", data.music.len())));
		}
		for c in data.music.iter() {
			if c.reason.as_ref().map_or(0, |f| f.len()) > 4096 {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", "理由过长".into()));
			}
			if c.first.unwrap_or_default() {
				if first_set {
					return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", "多个本命".into()));
				}
				first_set = true;
			}
			if chset.contains(&c.name) {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("{}已存在", c.name)));
			}
			chset.insert(c.name.clone());
		}
		// step 4: check all names are correct
		// step 5: return
		Ok(data)
	}
	pub async fn validate_cp(&self, mut data: models::CPSubmitRest, coll: &Collection<CPSubmitRest>) -> Result<models::CPSubmitRest, ServiceError> {
		let query = doc! {
			"meta.vote_id": data.meta.vote_id.clone()
		};
		// step 3: check ranks are unique from 1 to 6 and only one 本命
		let mut first_set = false;
		if data.cps.len() < 1 || data.cps.len() > 4 {
			return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("数量{}不在范围内[1,4]", data.cps.len())));
		}
		for c in data.cps.iter() {
			if c.reason.as_ref().map_or(0, |f| f.len()) > 4096 {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", "理由过长".into()));
			}
			if c.first.unwrap_or_default() {
				if first_set {
					return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", "多个本命".into()));
				}
				first_set = true;
			}
			if let Some(active) = &c.active {
				if *active != c.name_a && *active != c.name_b && {
					if let Some(name_c) = &c.name_c {
						*name_c != *active
					} else {
						true
					}
				} {
					return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("主动方{}不存在", active)));
				}
			}
		}
		// step 4: check all names are correct
		// step 5: return
		Ok(data)
	}
	pub async fn validate_paper(&self, data: models::PaperSubmitRest, coll: &Collection<PaperSubmitRest>) -> Result<models::PaperSubmitRest, ServiceError> {
		Ok(data)
	}
	pub async fn validate_dojin(&self, mut data: models::DojinSubmitRest, coll: &Collection<DojinSubmitRest>) -> Result<models::DojinSubmitRest, ServiceError> {
		for item in &data.dojins {
			if item.author.len() > 4096 {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("作者名过长")));
			}
			if item.reason.len() > 4096 {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("理由过长")));
			}
			if item.title.len() > 4096 {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("作品名过长")));
			}
			if item.url.len() > 4096 {
				return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_CONTENT", format!("URL过长")));
			}
		}
		Ok(data)
	}
}
