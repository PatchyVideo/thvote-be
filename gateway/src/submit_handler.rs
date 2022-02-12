
use juniper::IntoFieldError;
use juniper::graphql_value;

use juniper::FieldResult;
use pvrustlib::EmptyJSON;
use pvrustlib::ServiceError;
use pvrustlib::json_request_gateway;

use crate::common::SERVICE_NAME;
use crate::common::VoteTokenClaim;
use crate::context::Context;
use jwt_simple::{prelude::*, algorithms::ECDSAP256kPublicKeyLike};

use bson::DateTime;
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct SubmitMetadata {
	/// 投票ID，格式： thvote-{YYYY}-{phone|email}-{ID}
	pub vote_id: String,
	/// 提交时间
	pub created_at: bson::DateTime,
	/// 用户IP
	pub user_ip: String,
	/// 额外用户指纹信息
	pub additional_fingreprint: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRest {
	pub characters: Vec<CharacterSubmit>,
	pub meta: SubmitMetadata
}


#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRestQuery {
	pub characters: Vec<CharacterSubmitQuery>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MusicSubmitRest {
	pub music: Vec<MusicSubmit>,
	pub meta: SubmitMetadata
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct MusicSubmitRestQuery {
	pub music: Vec<MusicSubmitQuery>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkSubmitRest {
	pub works: Vec<WorkSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CPSubmitRest {
	pub cps: Vec<CPSubmit>,
	pub meta: SubmitMetadata
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct CPSubmitRestQuery {
	pub cps: Vec<CPSubmitQuery>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaperSubmitRest {
	pub papers_json: String,
	pub meta: SubmitMetadata
}

#[derive(juniper::GraphQLEnum, Debug, Clone, Serialize, Deserialize)]
pub enum DojinType {
	Music,
	Video,
	Drawing,
	Software,
	Article,
	Craft,
	Other
}


#[derive(Clone, Serialize, Deserialize)]
pub struct DojinSubmitRest {
	pub dojins: Vec<DojinSubmit>,
	pub meta: SubmitMetadata
}

#[derive(juniper::GraphQLInputObject, Debug, Clone, Serialize, Deserialize)]
pub struct DojinSubmit {
	pub dojin_type: DojinType,
	pub url: String,
	pub title: String,
	pub author: String,
	pub reason: String,
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct DojinSubmitQuery {
	pub dojin_type: DojinType,
	pub url: String,
	pub title: String,
	pub author: String,
	pub reason: String,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct DojinSubmitRestQuery {
	pub dojins: Vec<DojinSubmitQuery>,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct PaperSubmitRestQuery {
	pub papers_json: String,
}

// ------------------------------------------------
// GQL Schemas
// ------------------------------------------------

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single character submit")]
pub struct CharacterSubmit {
	/// 人物名
	pub name: String,
	/// 理由
	pub reason: Option<String>,
	/// 本命
	pub first: Option<bool>,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single character submit")]
pub struct CharacterSubmitQuery {
	/// 人物名
	pub name: String,
	/// 理由
	pub reason: Option<String>,
	/// 本命
	pub first: Option<bool>,
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Character submit")]
pub struct CharacterSubmitGQL {
	pub vote_token: String,
	pub characters: Vec<CharacterSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single cp submit")]
pub struct CPSubmit {
	/// 人物A
	pub name_a: String,
	/// 人物B
	pub name_b: String,
	/// 人物C（可选）
	pub name_c: Option<String>,
	/// 主动方（可选）
	pub active: Option<String>,
	/// 本命
	pub first: Option<bool>,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single cp submit")]
pub struct CPSubmitQuery {
	/// 人物A
	pub name_a: String,
	/// 人物B
	pub name_b: String,
	/// 人物C（可选）
	pub name_c: Option<String>,
	/// 主动方（可选）
	pub active: Option<String>,
	/// 本命
	pub first: Option<bool>,
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="CP submit")]
pub struct CPSubmitGQL {
	pub vote_token: String,
	pub cps: Vec<CPSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single music submit")]
pub struct MusicSubmit {
	/// 音乐名
	pub name: String,
	/// 理由
	pub reason: Option<String>,
	/// 本命
	pub first: Option<bool>,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single music submit")]
pub struct MusicSubmitQuery {
	/// 音乐名
	pub name: String,
	/// 理由
	pub reason: Option<String>,
	/// 本命
	pub first: Option<bool>,
}
#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Music submit")]
pub struct MusicSubmitGQL {
	pub vote_token: String,
	pub musics: Vec<MusicSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single work submit")]
pub struct WorkSubmit {
	/// 作品名
	pub name: String,
	/// 理由
	pub reason: Option<String>
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Work submit")]
pub struct WorkSubmitGQL {
	pub vote_token: String,
	pub work: Vec<WorkSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single paper submit")]
pub struct PaperSubmit {
	/// 提问ID
	pub id: String,
	/// 答案
	pub answer: String
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single paper submit")]
pub struct PaperSubmitQuery {
	/// 提问ID
	pub id: String,
	/// 答案
	pub answer: String
}


#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Paper submit")]
pub struct PaperSubmitGQL {
	/// 投票token
	pub vote_token: String,
	/// 问卷的JSON字符串
	pub paper_json: String
}

#[derive(Serialize, Deserialize)]
pub struct QuerySubmitRest {
	pub vote_id: String,
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Dojin submit")]
pub struct DojinSubmitGQL {
	pub vote_token: String,
	pub dojins: Vec<DojinSubmit>
}

#[derive(juniper::GraphQLObject, Clone, Debug, Serialize, Deserialize)]
#[graphql(description="投票进度")]
pub struct VotingStatus {
	/// 人物是否完成
	pub characters: bool,
	/// 音乐是否完成
	pub musics: bool,
	/// CP是否完成
	pub cps: bool,
	/// 问卷是否提交
	pub papers: bool,
}

pub fn generate_submit_metadata(vote_id: &str, context: &Context) -> SubmitMetadata {
	SubmitMetadata {
		vote_id: vote_id.to_string(),
		created_at: DateTime::now(),
		user_ip: context.user_ip.clone(),
		additional_fingreprint: None, // TODO
	}
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;

pub async fn submitCharacterVote_impl(context: &Context, content: &CharacterSubmitGQL) -> FieldResult<bool> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&content.vote_token, Some(options));
	println!("{:?}", result);
	if let Ok(claim) = result {
		let submit_json = CharacterSubmitRest {
			meta: generate_submit_metadata(&claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?, context),
			characters: content.characters.clone(),
		};
		
		let post_result: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/character/", SUBMIT_HANDLER), submit_json).await?;
		Ok(true)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn submitMusicVote_impl(context: &Context, content: &MusicSubmitGQL) -> FieldResult<bool> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&content.vote_token, Some(options));
	if let Ok(claim) = result {
		let submit_json = MusicSubmitRest {
			meta: generate_submit_metadata(&claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?, context),
			music: content.musics.clone(),
		};
		let post_result: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/music/", SUBMIT_HANDLER), submit_json).await?;
		Ok(true)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn submitCPVote_impl(context: &Context, content: &CPSubmitGQL) -> FieldResult<bool> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&content.vote_token, Some(options));
	if let Ok(claim) = result {
		let submit_json = CPSubmitRest {
			meta: generate_submit_metadata(&claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?, context),
			cps: content.cps.clone(),
		};
		let post_result: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/cp/", SUBMIT_HANDLER), submit_json).await?;
		Ok(true)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn submitPaperVote_impl(context: &Context, content: &PaperSubmitGQL) -> FieldResult<bool> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&content.vote_token, Some(options));
	if let Ok(claim) = result {
		let submit_json = PaperSubmitRest {
			meta: generate_submit_metadata(&claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?, context),
			papers_json: content.paper_json.clone()
		};
		let post_result: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/paper/", SUBMIT_HANDLER), submit_json).await?;
		Ok(true)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn submitDojinVote_impl(context: &Context, content: &DojinSubmitGQL) -> FieldResult<bool> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&content.vote_token, Some(options));
	if let Ok(claim) = result {
		let submit_json = DojinSubmitRest {
			meta: generate_submit_metadata(&claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?, context),
			dojins: content.dojins.clone()
		};
		let post_result: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/dojin/", SUBMIT_HANDLER), submit_json).await?;
		Ok(true)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}


pub async fn getSubmitCharacterVote_impl(context: &Context, vote_token: String) -> FieldResult<CharacterSubmitRestQuery> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&vote_token, Some(options));
	if let Ok(claim) = result {
		let query_json = QuerySubmitRest {
			vote_id: claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?
		};
		let post_result: CharacterSubmitRestQuery = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/get-character/", SUBMIT_HANDLER), query_json).await?;
		Ok(post_result)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn getSubmitMusicVote_impl(context: &Context, vote_token: String) -> FieldResult<MusicSubmitRestQuery> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&vote_token, Some(options));
	if let Ok(claim) = result {
		let query_json = QuerySubmitRest {
			vote_id: claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?
		};
		let post_result: MusicSubmitRestQuery = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/get-music/", SUBMIT_HANDLER), query_json).await?;
		Ok(post_result)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn getSubmitCPVote_impl(context: &Context, vote_token: String) -> FieldResult<CPSubmitRestQuery> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&vote_token, Some(options));
	if let Ok(claim) = result {
		let query_json = QuerySubmitRest {
			vote_id: claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?
		};
		let post_result: CPSubmitRestQuery = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/get-cp/", SUBMIT_HANDLER), query_json).await?;
		Ok(post_result)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn getSubmitPaperVote_impl(context: &Context, vote_token: String) -> FieldResult<PaperSubmitRestQuery> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&vote_token, Some(options));
	if let Ok(claim) = result {
		let query_json = QuerySubmitRest {
			vote_id: claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?
		};
		let post_result: PaperSubmitRestQuery = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/get-paper/", SUBMIT_HANDLER), query_json).await?;
		Ok(post_result)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}

pub async fn getSubmitDojinVote_impl(context: &Context, vote_token: String) -> FieldResult<DojinSubmitRestQuery> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&vote_token, Some(options));
	if let Ok(claim) = result {
		let query_json = QuerySubmitRest {
			vote_id: claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?
		};
		let post_result: DojinSubmitRestQuery = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/get-dojin/", SUBMIT_HANDLER), query_json).await?;
		Ok(post_result)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}


pub async fn getVotingStatus_impl(context: &Context, vote_token: String) -> FieldResult<VotingStatus> {
	let mut options = VerificationOptions::default();
	options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&vote_token, Some(options));
	if let Ok(claim) = result {
		let query_json = QuerySubmitRest {
			vote_id: claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?
		};
		let post_result: VotingStatus = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/voting-status/", SUBMIT_HANDLER), query_json).await?;
		Ok(post_result)
	} else {
		return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	}
}
