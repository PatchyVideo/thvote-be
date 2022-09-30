
use std::{collections::HashSet, hash::Hash};
use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde_derive::{Serialize, Deserialize};


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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPItem {
	pub a: String,
	pub b: String,
	pub c: Option<String>,
	pub active: Option<String>,
	pub reason: Option<String>
}

impl Hash for CPItem {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.a.hash(state);
		self.b.hash(state);
		self.c.hash(state);
	}
}

impl PartialEq for CPItem {
	fn eq(&self, other: &Self) -> bool {
		self.a == other.a && self.b == other.b && self.c == other.c
	}
}

impl Eq for CPItem {
	
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ValidQuestionnaireResponse {
	pub id: i32,
	pub answer: Option<HashSet<i32>>,
	pub answer_str: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingTrendItem {
	pub hrs: i32,
	pub cnt: i32
}

/// 用于角色和音乐
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

/// 用于CP
#[derive(Debug, Clone, Serialize, Deserialize)]
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
	/// 理由
	pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ReasonsRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	pub vote_start: DateTime<Utc>,
	pub vote_year: i32,
	pub rank: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TrendRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	pub vote_start: DateTime<Utc>,
	pub vote_year: i32,
	pub name: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReasonsResponse {
	pub reasons: Vec<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TrendResponse {
	pub trend: Vec<VotingTrendItem>,
	pub trend_first: Vec<VotingTrendItem>
}
#[derive(Clone, Serialize, Deserialize)]
pub struct RankingQueryResponse {
	pub entries: Vec<RankingEntry>,
	pub global: RankingGlobal
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CPRankingQueryResponse {
	pub entries: Vec<CPRankingEntry>,
	pub global: RankingGlobal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRankingEntry {
	pub key: String,
	pub vote_year: i32,
	pub entry: RankingEntry
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCPRankingEntry {
	pub key: String,
	pub vote_year: i32,
	pub entry: CPRankingEntry
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRankingGlobal {
	pub key: String,
	pub vote_year: i32,
	pub global: RankingGlobal
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RankingQueryRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub query: Option<String>,
	pub vote_start: DateTime<Utc>,
	pub vote_year: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialVoteItemEntry {
	pub vote_year: i32,
	pub name: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GlobalStatsRequest {
	pub vote_start: DateTime<Utc>,
	pub vote_year: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompletionRateItem {
	pub name: String,
	pub rate: f64,
	pub num_complete: i32,
	pub total: i32
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompletionRate {
	pub vote_year: i32,
	pub items: Vec<CompletionRateItem>
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CachedQuestionAnswerItem {
	pub aid: String,
	pub votes: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CachedQuestionItem {
	pub question_id: String,
	/// fixed set of choices
	pub answers_cat: Vec<CachedQuestionAnswerItem>,
	/// open-ended answers
	pub answers_str: Vec<String>,
	pub total_answers: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedQuestionEntry {
	pub key: String,
	pub vote_year: i32,
	pub entry: CachedQuestionItem,
	pub trend: Vec<VotingTrendItem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuestionnaireResponse {
	pub entries: Vec<CachedQuestionItem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinglePaperItem {
	pub opt: Vec<String>,
	pub ans: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct CachedCovote {
	pub key: String,
	pub vote_year: i32,
	pub first_k: i32, // only top k participate in covote calculation
	pub items: Vec<CovoteItem>
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovoteResponse {
	pub items: Vec<CovoteItem>
}
