

use juniper::IntoFieldError;
use juniper::graphql_value;

use juniper::FieldResult;
use pvrustlib::EmptyJSON;
use pvrustlib::ServiceError;
use pvrustlib::json_request_gateway;

use crate::common::SERVICE_NAME;
use crate::common::VoteTokenClaim;
use crate::context::Context;
use crate::services::RESULT_QUERY;
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
    pub entries: Vec<RankingEntry>,
	/// 角色数/音乐数
	pub total_unique_items: i32,
	/// 总本命数
	pub total_first: i32,
	/// 总票数
	pub total_votes: i32,
	/// 全角色平均票数
	pub average_votes_per_item: f64,
	/// 全角色中位票数
	pub median_votes_per_item: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RankingQueryRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
	pub query: Option<String>,
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>
}


pub async fn queryCharacterRanking_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>) -> FieldResult<CharacterOrMusicRanking> {
	let query_json = RankingQueryRequest {
		query,
		vote_start
	};
	let post_result: CharacterOrMusicRanking = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/chars-rank/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryMusicRanking_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>) -> FieldResult<CharacterOrMusicRanking> {
    let query_json = RankingQueryRequest {
		query,
		vote_start
	};
	let post_result: CharacterOrMusicRanking = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/musics-rank/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct Reasons {
    pub items: Vec<String>
}

pub async fn listReasonsCharacter_impl(context: &Context, name: String) -> FieldResult<Reasons> {
    Ok(Reasons {items: vec![]})
}
