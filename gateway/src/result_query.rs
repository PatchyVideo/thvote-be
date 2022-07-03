

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

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VotingTrendItem {
	pub date: DateTime<Utc>,
	pub vote_count: i32
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
/// 用于角色和音乐
pub struct RankingEntry {
	/// 排名
	pub rank: i32,
	/// 角色名
	pub name: String,
	/// 票数
	pub vote_count: i32,
	/// 本命票数
	pub first_vote_count: i32,
	/// 本命率
	pub first_vote_percentage: f64,
	/// 本命甲醛
	pub first_vote_count_weighted: i32,
	/// 所属作品类型
	pub character_type: String,
	/// 所属作品
	pub character_origin: String,
	/// 初登场时间
	pub first_appearance: String,
	/// 日文名
	pub name_jpn: String,
	/// 票数占比
	pub vote_percentage: f64,
	/// 本命占比
	pub first_percentage: f64,
	/// 男性票数
	pub male_vote_count: i32,
	/// 男性比例
	pub male_percentage_per_char: f64,
	/// 占总体男性比例
	pub male_percentage_per_total: f64,
	/// 女性票数
	pub female_vote_count: i32,
	/// 女性比例
	pub female_percentage_per_char: f64,
	/// 占总体女性比例
	pub female_percentage_per_total: f64,
	/// 趋势
	pub trend: Vec<VotingTrendItem>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct CharacterOrMusicRanking {
    pub items: Vec<RankingEntry>
}

pub async fn queryCharacterRanking_impl(context: &Context, query: Option<String>) -> FieldResult<CharacterOrMusicRanking> {
	// let mut options = VerificationOptions::default();
	// options.allowed_audiences = Some(HashSet::from_strings(&["vote"]));
	// let result = context.public_key.public_key().verify_token::<VoteTokenClaim>(&vote_token, Some(options));
	// if let Ok(claim) = result {
	// 	let query_json = QuerySubmitRest {
	// 		vote_id: claim.custom.vote_id.ok_or(ServiceError::new_jwt_error(SERVICE_NAME, None))?
	// 	};
	// 	let post_result: VotingStatus = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/voting-status/", SUBMIT_HANDLER), query_json).await?;
	// 	Ok(post_result)
	// } else {
	// 	return Err(ServiceError::new_jwt_error(SERVICE_NAME, None).into_field_error());
	// }
    Ok(CharacterOrMusicRanking {items: vec![]})
}

pub async fn queryMusicRanking_impl(context: &Context, query: Option<String>) -> FieldResult<CharacterOrMusicRanking> {
    Ok(CharacterOrMusicRanking {items: vec![]})
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct Reasons {
    pub items: Vec<String>
}

pub async fn listReasonsCharacter_impl(context: &Context, name: String) -> FieldResult<Reasons> {
    Ok(Reasons {items: vec![]})
}
