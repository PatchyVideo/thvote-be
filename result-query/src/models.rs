
use std::collections::HashSet;
use chrono::{DateTime, Utc};
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


#[derive(Clone, Serialize, Deserialize)]
pub struct ValidQuestionnaireResponse {
	pub id: i32,
	pub answer: Option<HashSet<i32>>,
	pub answer_str: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VotingTrendItem {
	pub hrs: i32,
	pub vote_count: i32
}

/// 用于角色和音乐
#[derive(Clone, Serialize, Deserialize)]
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
	pub trend: Vec<VotingTrendItem>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RankingMeta {
	/// 角色数/音乐数
	pub total_unique_items: i32,
	/// 总本命数
	pub total_first: i32,
	/// 总票数
	pub total_votes: i32,
	/// 全角色平均票数
	pub average_votes_per_item: f64,
	/// 全角色中位票数
	pub median_votes_per_item: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RankingQueryResponse {
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
	pub vote_start: DateTime<Utc>
}
