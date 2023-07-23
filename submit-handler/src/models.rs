
use bson;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteTokenClaim {
	pub vote_id: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitMetadata {
	/// 投票人id
	pub vote_id: String,
	/// 这是第几次提交该问卷（由本程序生成，无需提交）
	pub attempt: Option<i32>,
	/// 提交时间
	pub created_at: bson::DateTime,
	/// 用户IP
	pub user_ip: String,
	/// 额外用户指纹信息
	pub additional_fingreprint: Option<String>
}
impl SubmitMetadata {
	pub fn new() -> SubmitMetadata {
		SubmitMetadata {
			vote_id: "<unknown>".into(),
			attempt: None,
			created_at: bson::DateTime::now(),
			user_ip: "<unknown>".into(),
			additional_fingreprint: None
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRest {
	pub characters: Vec<CharacterSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSubmitRest {
	pub music: Vec<MusicSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSubmitRest {
	pub works: Vec<WorkSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPSubmitRest {
	pub cps: Vec<CPSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperSubmitRest {
	pub papers_json: String,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DojinSubmitRest {
	pub dojins: Vec<DojinSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DojinSubmit {
	pub dojin_type: String,
	pub url: String,
	pub title: String,
	pub author: String,
	pub reason: String,
	pub image_url: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmit {
	pub id: String,
	pub reason: Option<String>,
	pub first: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPSubmit {
	pub id_a: String,
	pub id_b: String,
	pub id_c: Option<String>,
	pub active: Option<String>,
	pub first: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
	pub reason: Option<String>
}

impl PartialEq for CPSubmit {
    fn eq(&self, other: &Self) -> bool {
        if self.id_c.is_some() ^ other.id_c.is_some() {
			return false;
		}
        if self.active.is_some() ^ other.active.is_some() {
			return false;
		}
		if let (Some(c1), Some(c2)) = (&self.active, &other.active) {
			if c1 != c2 {
				return false;
			}
		}
		if let (Some(c1), Some(c2)) = (&self.id_c, &other.id_c) {
			((self.id_a == other.id_a) && (self.id_b == other.id_b) && (*c1 == *c2)) ||
			((self.id_a == other.id_b) && (self.id_b == other.id_a) && (*c1 == *c2)) ||
			((self.id_a == *c2) && (self.id_b == other.id_b) && (*c1 == other.id_a)) ||
			((self.id_a == other.id_a) && (self.id_b == *c2) && (*c1 == other.id_b))
		} else {
			((self.id_a == other.id_a) && (self.id_b == other.id_b)) ||
			((self.id_a == other.id_b) && (self.id_b == other.id_a))
		}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSubmit {
	pub id: String,
	pub reason: Option<String>,
	pub first: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSubmit {
	pub id: String,
	pub reason: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySubmitRequest {
	pub vote_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingStatus {
	/// 人物是否完成
	pub characters: bool,
	/// 音乐是否完成
	pub musics: bool,
	/// CP是否完成
	pub cps: bool,
	/// 问卷是否提交
	pub papers: bool,
	/// 同人作品是否提交
	pub dojin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingStatistics {
	pub num_user: u64,
	pub num_finished_paper: u64,
	pub num_finished_voting: u64,
	pub num_character: u64,
	pub num_cp: u64,
	pub num_music: u64,
	pub num_dojin: u64,
}
