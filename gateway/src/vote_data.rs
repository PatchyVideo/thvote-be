

use juniper::FieldResult;
use serde_derive::{Serialize, Deserialize};

use crate::context::Context;

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VotableCharacters {
    pub data: Vec<VotableCharacter>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VotableCharacter {
    pub name: String,
    pub altnames: Vec<String>,
    pub title: Option<String>,
    pub image: String,
    pub color: String,
    /// 出场作品
    pub appeared_in: Vec<String>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VotableMusics {
    pub data: Vec<VotableMusic>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VotableMusic {
    pub name: String,
    pub altnames: Vec<String>,
    pub image: String,
    /// 首次出场的作品
    pub first_appeared_in: String,
    pub character: Option<String>
}


#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VotableWorks {
    pub data: Vec<VotableWork>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VotableWork {
    pub name: String,
    pub altnames: Vec<String>,
    pub image: String,
    /// 发布日期
    pub release_date: String
}

pub async fn listVotableCharacters_impl(context: &Context) -> FieldResult<VotableCharacters> {
	Ok(VotableCharacters {
        data: vec![
            VotableCharacter {
                name: "博丽灵梦".to_string(),
                altnames: vec!["城管".to_string(), "红白".to_string(), "reimu".to_string()],
                title: Some("乐园的可爱巫女".to_string()),
                image: "none".to_string(),
                color: "FF0000".to_string(),
                appeared_in: vec!["东方虹龙洞".to_string(), "等等".to_string()]
            }
        ]
    })
}

pub async fn listVotableWorks_impl(context: &Context) -> FieldResult<VotableWorks> {
	Ok(VotableWorks {
        data: vec![
            VotableWork {
                name: "东方虹龙洞".to_string(),
                altnames: vec!["um".to_string(), "Unconnected Marketeers".to_string()],
                image: "none".to_string(),
                release_date: "2021-05-04".to_string()
            }
        ]
    })
}

pub async fn listVotableMusics_impl(context: &Context) -> FieldResult<VotableMusics> {
	Ok(VotableMusics {
        data: vec![
            VotableMusic {
                name: "大吉猫咪".to_string(),
                altnames: vec!["Kitten of Great Fortune".to_string(), "大吉キトゥン".to_string()],
                image: "none".to_string(),
                first_appeared_in: "东方虹龙洞".to_string(),
                character: Some("豪德寺三花".to_string())
            }
        ]
    })
}
