use std::collections::HashSet;

use bson::{doc, oid::ObjectId};
use futures_util::{TryStreamExt};
use mongodb::{Collection, Database};
use pvrustlib::ServiceError;
use redlock::RedLock;

use crate::models::{CPSubmitRest, CharacterSubmitRest, MusicSubmitRest, PaperSubmitRest, WorkSubmitRest, VotingStatus, SubmitMetadata, DojinSubmitRest, VotingStatistics};
use crate::{models, validator};
use crate::common::{SERVICE_NAME};

#[derive(Clone)]
pub struct SubmitServiceV1 {
	pub character_coll: Collection<CharacterSubmitRest>,
	pub music_coll: Collection<MusicSubmitRest>,
	pub cp_coll: Collection<CPSubmitRest>,
	pub work_coll: Collection<WorkSubmitRest>,
	pub paper_coll: Collection<PaperSubmitRest>,
	pub dojin_coll: Collection<DojinSubmitRest>,
	pub validator: validator::SubmitValidatorV1,
	pub lock: RedLock,
	pub redis_client: redis::Client
}

impl SubmitServiceV1 {
	pub async fn new(db: Database, redis: redis::Client, lock: RedLock) -> SubmitServiceV1 {
		SubmitServiceV1 { 
			character_coll: db.collection::<CharacterSubmitRest>("raw_character"),
			music_coll: db.collection::<MusicSubmitRest>("raw_music"),
			cp_coll: db.collection::<CPSubmitRest>("raw_cp"),
			work_coll: db.collection::<WorkSubmitRest>("raw_work"),
			paper_coll: db.collection::<PaperSubmitRest>("raw_paper"),
			dojin_coll: db.collection::<DojinSubmitRest>("raw_dojin"),
			validator: validator::SubmitValidatorV1::new().await,
			lock: lock,
			redis_client: redis
		}
	}

	pub async fn submit_charcater(&self, verified_data: models::CharacterSubmitRest) -> Result<ObjectId, ServiceError> {
		match self.character_coll.insert_one(verified_data.clone(), None).await {
			Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
			Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
		}
	}

	pub async fn get_submit_charcater(&self, vote_id: String) -> Result<CharacterSubmitRest, ServiceError> {
		let stages = vec![
			doc!{"$match": {"meta.vote_id": vote_id}},
			doc!{"$sort": {"meta.created_at": -1}}
		];
		let mut cursor = self.character_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		match submit {
			Some(submit) => {
				let mut submit: CharacterSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
				submit.meta.additional_fingreprint = None;
				submit.meta.user_ip = "".to_string();
				submit.meta.vote_id = "".to_string();
				Ok(submit)
			},
			None => {
				Ok(CharacterSubmitRest {
					characters: vec![],
					meta: SubmitMetadata::new()
				})
			},
		}
	}

	pub async fn submit_music(&self, verified_data: models::MusicSubmitRest) -> Result<ObjectId, ServiceError> {
		match self.music_coll.insert_one(verified_data.clone(), None).await {
			Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
			Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
		}
	}

	pub async fn get_submit_music(&self, vote_id: String) -> Result<MusicSubmitRest, ServiceError> {
		let stages = vec![
			doc!{"$match": {"meta.vote_id": vote_id}},
			doc!{"$sort": {"meta.created_at": -1}}
		];
		let mut cursor = self.music_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		match submit {
			Some(submit) => {
				let mut submit: MusicSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
				submit.meta.additional_fingreprint = None;
				submit.meta.user_ip = "".to_string();
				submit.meta.vote_id = "".to_string();
				Ok(submit)
			},
			None => {
				Ok(MusicSubmitRest {
					music: vec![],
					meta: SubmitMetadata::new()
				})
			},
		}
	}

	
	pub async fn submit_cp(&self, verified_data: models::CPSubmitRest) -> Result<ObjectId, ServiceError> {
		match self.cp_coll.insert_one(verified_data.clone(), None).await {
			Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
			Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
		}
	}

	pub async fn get_submit_cp(&self, vote_id: String) -> Result<CPSubmitRest, ServiceError> {
		let stages = vec![
			doc!{"$match": {"meta.vote_id": vote_id}},
			doc!{"$sort": {"meta.created_at": -1}}
		];
		let mut cursor = self.cp_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		match submit {
			Some(submit) => {
				let mut submit: CPSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
				submit.meta.additional_fingreprint = None;
				submit.meta.user_ip = "".to_string();
				submit.meta.vote_id = "".to_string();
				Ok(submit)
			},
			None => {
				Ok(CPSubmitRest {
					cps: vec![],
					meta: SubmitMetadata::new()
				})
			},
		}
	}

	pub async fn submit_paper(&self, verified_data: models::PaperSubmitRest) -> Result<ObjectId, ServiceError> {
		match self.paper_coll.insert_one(verified_data.clone(), None).await {
			Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
			Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
		}
	}

	pub async fn get_submit_paper(&self, vote_id: String) -> Result<PaperSubmitRest, ServiceError> {
		let stages = vec![
			doc!{"$match": {"meta.vote_id": vote_id}},
			doc!{"$sort": {"meta.created_at": -1}}
		];
		let mut cursor = self.paper_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		match submit {
			Some(submit) => {
				let mut submit: PaperSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
				submit.meta.additional_fingreprint = None;
				submit.meta.user_ip = "".to_string();
				submit.meta.vote_id = "".to_string();
				Ok(submit)
			},
			None => {
				Ok(PaperSubmitRest {
					papers_json: "{}".into(),
					meta: SubmitMetadata::new()
				})
			},
		}
	}

	pub async fn submit_dojin(&self, verified_data: models::DojinSubmitRest) -> Result<ObjectId, ServiceError> {
		match self.dojin_coll.insert_one(verified_data.clone(), None).await {
			Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
			Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
		}
	}

	pub async fn get_submit_dojin(&self, vote_id: String) -> Result<DojinSubmitRest, ServiceError> {
		let stages = vec![
			doc!{"$match": {"meta.vote_id": vote_id}},
			doc!{"$sort": {"meta.created_at": -1}}
		];
		let mut cursor = self.dojin_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		match submit {
			Some(submit) => {
				let mut submit: DojinSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
				submit.meta.additional_fingreprint = None;
				submit.meta.user_ip = "".to_string();
				submit.meta.vote_id = "".to_string();
				Ok(submit)
			},
			None => {
				Ok(DojinSubmitRest {
					dojins: vec![],
					meta: SubmitMetadata::new()
				})
			},
		}
	}

	pub async fn get_voting_status(&self, vote_id: String) -> Result<VotingStatus, ServiceError> {
		let stages = vec![
			doc!{"$match": {"meta.vote_id": vote_id}},
			doc!{"$sort": {"meta.created_at": -1}}
		];
		
		let ch = {
			let mut cursor = self.character_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
		};
		let music = {
			let mut cursor = self.music_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
		};
		let cp = {
			let mut cursor = self.cp_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
		};
		let paper = {
			let mut cursor = self.paper_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
		};
		let dojin = {
			let mut cursor = self.dojin_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
		};
		Ok(VotingStatus {
			characters: ch,
			musics: music,
			cps: cp,
			papers: paper,
			dojin: dojin
		})
	}
	pub async fn get_voting_statistics(&self) -> Result<VotingStatistics, ServiceError> {
		let all_ch_voter = self.character_coll.distinct("meta.vote_id", doc!{}, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let all_cp_voter = self.cp_coll.distinct("meta.vote_id", doc!{}, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let all_music_voter = self.music_coll.distinct("meta.vote_id", doc!{}, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut all_voters = HashSet::new();
		for voter in all_ch_voter.iter() {
			let item: CharacterSubmitRest = bson::from_bson(voter.clone()).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			all_voters.insert(item.meta.vote_id);
		}
		for voter in all_cp_voter.iter() {
			let item: CPSubmitRest = bson::from_bson(voter.clone()).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			all_voters.insert(item.meta.vote_id);
		}
		for voter in all_music_voter.iter() {
			let item: MusicSubmitRest = bson::from_bson(voter.clone()).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			all_voters.insert(item.meta.vote_id);
		}
		let mut all_voters_inc_paper = all_voters.clone();
		let all_paper_voter = self.paper_coll.distinct("meta.vote_id", doc!{}, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let all_dojin_voter = self.dojin_coll.distinct("meta.vote_id", doc!{}, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		for voter in all_paper_voter.iter() {
			let item: PaperSubmitRest = bson::from_bson(voter.clone()).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
			all_voters_inc_paper.insert(item.meta.vote_id);
		}
		Ok(VotingStatistics {
			num_user: all_voters_inc_paper.len() as _,
			num_finished_paper: 0,
			num_finished_voting: all_voters.len() as _,
			num_character: all_ch_voter.len() as _,
			num_cp: all_cp_voter.len() as _,
			num_music: all_music_voter.len() as _,
			num_dojin: all_dojin_voter.len() as _,
		})
	}
}
