
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
	/// 角色名
	pub name: String,
	/// 票数
	pub vote_count: i32,
	/// 本命票数
	pub first_vote_count: i32,
	/// 本命率
	pub first_vote_percentage: f64,
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

/// 用于CP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPRankingEntry {
	/// 排名
	pub rank: i32,
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
pub struct ReasonsResponse {
	pub reasons: Vec<String>
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
