

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

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct VotingTrendItem {
	pub hrs: i32,
	pub cnt: i32
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
/// 用于角色和音乐
pub struct RankingEntry {
	/// 排名
	pub rank: i32,
	/// 排名上一届
	pub rank_last_1: i32,
	/// 排名上上届
	pub rank_last_2: i32,
	/// 展示排名
	pub display_rank: i32,
	/// 角色名
	pub name: String,
	/// 票数
	pub vote_count: i32,
	/// 票数
	pub vote_count_last_1: i32,
	/// 票数
	pub vote_count_last_2: i32,
	/// 本命票数
	pub first_vote_count: i32,
	/// 本命票数上一届
	pub first_vote_count_last_1: i32,
	/// 本命票数上上届
	pub first_vote_count_last_2: i32,
	/// 本命率
	pub first_vote_percentage: f64,
	/// 本命率上一届
	pub first_vote_percentage_last_1: f64,
	/// 本命率上上届
	pub first_vote_percentage_last_2: f64,
	/// 本命加权
	pub first_vote_count_weighted: i32,
	/// 所属作品类型
	pub character_type: String,
	/// 所属作品
	pub character_origin: String,
	/// 初登场时间
	pub first_appearance: String,
	/// 专辑
	pub album: Option<String>,
	/// 日文名
	pub name_jpn: String,
	/// 票数占比
	pub vote_percentage: f64,
	/// 票数占比上一届
	pub vote_percentage_last_1: f64,
	/// 票数占比上上届
	pub vote_percentage_last_2: f64,
	/// 本命占比
	pub first_percentage: f64,
	/// 男性票数
	pub male_vote_count: i32,
	/// 男性比例 P(male|voted)
	pub male_percentage_per_char: f64,
	/// 占总体男性比例 P(voted|male)
	pub male_percentage_per_total: f64,
	/// 女性票数
	pub female_vote_count: i32,
	/// 女性比例
	pub female_percentage_per_char: f64,
	/// 占总体女性比例
	pub female_percentage_per_total: f64,
	/// 趋势
	pub trend: Vec<VotingTrendItem>,
	/// 本命趋势
	pub trend_first: Vec<VotingTrendItem>,
	/// 理由
	pub reasons: Vec<String>,
	/// 理由数量
	pub num_reasons: i32,
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct CPItem {
	pub a: String,
	pub b: String,
	pub c: Option<String>
}

/// 用于CP
#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct CPRankingEntry {
	/// 排名
	pub rank: i32,
	/// 展示排名
	pub display_rank: i32,
	/// 角色名
	pub cp: CPItem,
	/// A主动率
	pub a_active: f64,
	/// B主动率
	pub b_active: f64,
	/// C主动率
	pub c_active: f64,
	/// 无主动率
	pub none_active: f64,
	/// 票数
	pub vote_count: i32,
	/// 本命票数
	pub first_vote_count: i32,
	/// 本命率
	pub first_vote_percentage: f64,
	/// 本命加权
	pub first_vote_count_weighted: i32,
	/// 票数占比
	pub vote_percentage: f64,
	/// 本命占比
	pub first_percentage: f64,
	/// 男性票数
	pub male_vote_count: i32,
	/// 男性比例 P(male|voted)
	pub male_percentage_per_char: f64,
	/// 占总体男性比例 P(voted|male)
	pub male_percentage_per_total: f64,
	/// 女性票数
	pub female_vote_count: i32,
	/// 女性比例
	pub female_percentage_per_char: f64,
	/// 占总体女性比例
	pub female_percentage_per_total: f64,
	/// 趋势
	pub trend: Vec<VotingTrendItem>,
	/// 本命趋势
	pub trend_first: Vec<VotingTrendItem>,
	/// 理由
	pub reasons: Vec<String>,
	/// 理由数量
	pub num_reasons: i32,
}


#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct RankingGlobal {
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

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct CharacterOrMusicRanking {
	pub entries: Vec<RankingEntry>,
	pub global: RankingGlobal
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct CPRanking {
	pub entries: Vec<CPRankingEntry>,
	pub global: RankingGlobal
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RankingQueryRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>,
	/// 第几届
	pub vote_year: i32
}


#[derive(Clone, Serialize, Deserialize)]
pub struct ReasonsRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>,
	/// 第几届
	pub vote_year: i32,
	/// 排名
	pub rank: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TrendRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>,
	/// 第几届
	pub vote_year: i32,
	/// 名字或问题ID，q开头
	pub name: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TrendRequestRank {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	pub vote_start: DateTime<Utc>,
	pub vote_year: i32,
	pub rank: i32
}


#[derive(Clone, Serialize, Deserialize)]
pub struct GlobalStatsRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>,
	/// 第几届
	pub vote_year: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompletionRateRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>,
	/// 第几届
	pub vote_year: i32,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct Reasons {
	pub reasons: Vec<String>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct Trends {
	pub trend: Vec<VotingTrendItem>,
	pub trend_first: Vec<VotingTrendItem>
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
	pub vote_year: i32,
	pub num_vote: i32,
	pub num_char: i32,
	pub num_music: i32,
	pub num_cp: i32,
	pub num_doujin: i32,
	pub num_male: i32,
	pub num_female: i32,
}


#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompletionRateItem {
	pub name: String,
	pub rate: f64,
	pub num_complete: i32,
	pub total: i32
}


#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompletionRate {
	pub vote_year: i32,
	pub items: Vec<CompletionRateItem>
}


#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CachedQuestionAnswerItem {
	pub aid: String,
	pub total_votes: i32,
	pub male_votes: i32,
	pub female_votes: i32
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CachedQuestionItem {
	pub question_id: String,
	/// fixed set of choices
	pub answers_cat: Vec<CachedQuestionAnswerItem>,
	/// open-ended answers
	pub answers_str: Vec<String>,
	pub total_answers: i32,
	pub total_male: i32,
	pub total_female: i32,
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct CachedQuestionEntry {
	pub key: String,
	pub vote_year: i32,
	pub entry: CachedQuestionItem
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuestionnaireRequest {
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>,
	/// 第几届
	pub vote_year: i32,
	/// 要查询哪几个问题ID，q开头
	pub questions_of_interest: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuestionnaireResponse {
	pub entries: Vec<CachedQuestionItem>
}


#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct CovoteItem {
	pub a: String,
	pub b: String,
	/// chi_square
	pub cs: f64,
	/// mutual_info
	pub mi: f64,
	/// co vote rate (V1∩V2 / V1∪V2)
	pub cv: f64,
	pub m00: i32,
	pub m01: i32,
	pub m10: i32,
	pub m11: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovoteRequest {
	/// 投票开始时间，UTC
	pub vote_start: DateTime<Utc>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	pub vote_year: i32,
	pub first_k: i32
}

#[derive(juniper::GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct CovoteResponse {
	pub items: Vec<CovoteItem>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SingleRankQuery {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	pub vote_start: DateTime<Utc>,
	pub vote_year: i32,
	pub rank: i32
}

pub async fn queryCharacterRanking_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<CharacterOrMusicRanking> {
	let query_json = RankingQueryRequest {
		query,
		vote_start,
		vote_year
	};
	let post_result: CharacterOrMusicRanking = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/chars-rank/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCharacterReasons_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<Reasons> {
	let query_json = ReasonsRequest {
		query,
		vote_start,
		vote_year,
		rank
	};
	let post_result: Reasons = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/chars-reasons/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCharacterSingle_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<RankingEntry> {
	let query_json = SingleRankQuery {
		query,
		vote_start,
		vote_year,
		rank
	};
	let post_result: RankingEntry = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/chars-single/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCharacterTrend_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, name: String) -> FieldResult<Trends> {
	let query_json = TrendRequest {
		query,
		vote_start,
		vote_year,
		name
	};
	let post_result: Trends = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/chars-trend/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryMusicRanking_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<CharacterOrMusicRanking> {
	let query_json = RankingQueryRequest {
		query,
		vote_start,
		vote_year
	};
	let post_result: CharacterOrMusicRanking = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/musics-rank/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryMusicReasons_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<Reasons> {
	let query_json = ReasonsRequest {
		query,
		vote_start,
		vote_year,
		rank
	};
	let post_result: Reasons = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/musics-reasons/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryMusicSingle_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<RankingEntry> {
	let query_json = SingleRankQuery {
		query,
		vote_start,
		vote_year,
		rank
	};
	let post_result: RankingEntry = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/musics-single/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryMusicTrend_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, name: String) -> FieldResult<Trends> {
	let query_json = TrendRequest {
		query,
		vote_start,
		vote_year,
		name
	};
	let post_result: Trends = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/musics-trend/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCPRanking_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<CPRanking> {
	let query_json = RankingQueryRequest {
		query,
		vote_start,
		vote_year
	};
	let post_result: CPRanking = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/cps-rank/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCPReasons_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<Reasons> {
	let query_json = ReasonsRequest {
		query,
		vote_start,
		vote_year,
		rank
	};
	let post_result: Reasons = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/cps-reasons/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCPSingle_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<CPRankingEntry> {
	let query_json = SingleRankQuery {
		query,
		vote_start,
		vote_year,
		rank
	};
	let post_result: CPRankingEntry = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/cps-single/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCPTrend_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<Trends> {
	let query_json = TrendRequestRank {
		query,
		vote_start,
		vote_year,
		rank
	};
	let post_result: Trends = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/cps-trend/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryGlobalStats_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<GlobalStats> {
	let query_json = GlobalStatsRequest {
		vote_start,
		vote_year,
		query
	};
	let post_result: GlobalStats = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/global-stats/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCompletionRate_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<CompletionRate> {
	let query_json = CompletionRateRequest {
		vote_start,
		vote_year,
		query
	};
	let post_result: CompletionRate = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/completion-rates/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryQuestionnaire_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, questions_of_interest: Vec<String>) -> FieldResult<QueryQuestionnaireResponse> {
	let query_json = QueryQuestionnaireRequest {
		query,
		vote_start,
		vote_year,
		questions_of_interest: questions_of_interest
	};
	let post_result: QueryQuestionnaireResponse = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/papers/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryQuestionnaireTrend_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, name: String) -> FieldResult<Trends> {
	let query_json = TrendRequest {
		query,
		vote_start,
		vote_year,
		name
	};
	let post_result: Trends = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/papers-trend/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryCharsCovote_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, top_k: i32) -> FieldResult<CovoteResponse> {
	let query_json = CovoteRequest {
		query,
		vote_start,
		vote_year,
		first_k: top_k
	};
	let post_result: CovoteResponse = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/chars-covote/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}

pub async fn queryMusicsCovote_impl(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, top_k: i32) -> FieldResult<CovoteResponse> {
	let query_json = CovoteRequest {
		query,
		vote_start,
		vote_year,
		first_k: top_k
	};
	let post_result: CovoteResponse = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/musics-covote/", RESULT_QUERY), query_json).await?;
	Ok(post_result)
}
