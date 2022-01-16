
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
pub struct CharacterSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub first: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPSubmit {
	pub name_a: String,
	pub name_b: String,
	pub name_c: Option<String>,
	pub active: Option<String>,
	pub first: Option<bool>
}

impl PartialEq for CPSubmit {
    fn eq(&self, other: &Self) -> bool {
        if self.name_c.is_some() ^ other.name_c.is_some() {
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
		if let (Some(c1), Some(c2)) = (&self.name_c, &other.name_c) {
			((self.name_a == other.name_a) && (self.name_b == other.name_b) && (*c1 == *c2)) ||
			((self.name_a == other.name_b) && (self.name_b == other.name_a) && (*c1 == *c2)) ||
			((self.name_a == *c2) && (self.name_b == other.name_b) && (*c1 == other.name_a)) ||
			((self.name_a == other.name_a) && (self.name_b == *c2) && (*c1 == other.name_b))
		} else {
			((self.name_a == other.name_a) && (self.name_b == other.name_b)) ||
			((self.name_a == other.name_b) && (self.name_b == other.name_a))
		}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub first: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSubmit {
	pub name: String,
	pub reason: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySubmitRequest {
	pub vote_id: String
}

#[derive( Clone, Serialize, Deserialize)]
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

// 人物部分
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   charcaters:[{
//     name: '',
//     reason: '', // 理由
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// CP部分
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   cps:[{
//     char1: '',
//     char2: '',
//     active: '', // 主动方
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 音乐部分：
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   musics:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 作品部分：
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   works:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 问卷部分
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   items:[{
//     item: '' //问卷项代码
//     answer: '' //回答内容
//   }, ...]
// }
