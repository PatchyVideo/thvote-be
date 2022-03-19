
// use juniper::graphql_value;
// use std::collections::BTreeMap;

// use juniper::FieldResult;

// use crate::common::*;

// use serde_derive::{Serialize, Deserialize};
// use bson::oid::ObjectId;

// // ------------------------------------------------
// // REST Schemas
// // ------------------------------------------------

// #[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
// #[graphql(description="单个问卷项目选项及对应结果")]
// pub struct SinglePaperOptionResult {
//     /// 选项名
//     pub option_id: String,
//     /// 结果数量（男）
//     pub result_male: i32,
//     /// 结果数量（女）
//     pub result_female: i32
// }

// #[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
// #[graphql(description="单个问卷项目结果")]
// pub struct SinglePaperResult {
//     /// 项目名
//     pub id: String,
//     /// 回答结果（数值结果）
//     pub answers_counts: Option<Vec<SinglePaperOptionResult>>,
//     /// 回答结果（文本结果）
//     pub answers_texts: Option<Vec<String>>
// }

// #[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
// #[graphql(description="问卷项目结果")]
// pub struct PaperResults {
//     /// 所有问卷项目结果
//     pub results: Vec<SinglePaperResult>,
//     /// 使用的过滤器
//     pub filter_condtions: Option<FilterConditionsOutput>,
//     /// 填写趋势
//     pub trends: Option<Trends>,
// }

// #[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
// #[graphql(description="单个过滤器条件")]
// pub struct SingleFilterCondition {
//     /// 来源
//     pub section: VoteSection, 
//     /// 条件
//     pub condition: FilterConditionOp, 
//     /// 左值
//     pub lhs: String, 
//     /// 右值
//     pub rhs: String 
// }

// #[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
// #[graphql(description="过滤器条件（所有条件与）")]
// pub struct FilterConditions { // 
//     pub conditions: Vec<SingleFilterCondition>
// }

// #[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
// #[graphql(description="(输出)单个过滤器条件")]
// pub struct SingleFilterConditionOutput {
//     /// 来源
//     pub section: VoteSection, 
//     /// 条件
//     pub condition: FilterConditionOp, 
//     /// 左值
//     pub lhs: String, 
//     /// 右值
//     pub rhs: String 
// }

// #[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
// #[graphql(description="(输出)过滤器条件（所有条件与）")]
// pub struct FilterConditionsOutput { // 
//     pub conditions: Vec<SingleFilterConditionOutput>
// }

// impl SingleFilterConditionOutput {
//     pub fn from_input(inp: &SingleFilterCondition) -> SingleFilterConditionOutput {
//         SingleFilterConditionOutput {
//             section: inp.section.clone(),
//             condition: inp.condition.clone(),
//             lhs: inp.lhs.clone(),
//             rhs: inp.rhs.clone()
//         }
//     }
// }

// impl FilterConditionsOutput {
//     pub fn from_input(inp: &FilterConditions) -> FilterConditionsOutput {
//         FilterConditionsOutput {
//             conditions: inp.conditions.iter().map(|ref x| SingleFilterConditionOutput::from_input(x)).collect::<Vec<_>>()
//         }
//     }
// }

// #[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
// #[graphql(description="投票理由集合")]
// pub struct Reasons {
//     pub reasons: Vec<String>
// }

// #[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
// #[graphql(description="投票时间趋势，返回开始到结束每小时的票数")]
// pub struct Trends {
//     /// 新增票数
//     pub vote_inc: Option<Vec<i32>>,
//     /// 减少票数
//     pub vote_dec: Option<Vec<i32>>,
//     /// 新增本命
//     pub first_inc: Option<Vec<i32>>,
//     /// 减少本命
//     pub first_dev: Option<Vec<i32>>,
//     /// 总票数
//     pub vote_cum: Option<Vec<i32>>,
//     /// 总本命数
//     pub first_cum: Option<Vec<i32>>,
//     /// 开始时间
//     pub from_date: DateTime<Utc>,
//     /// 结束时间
//     pub to_date: DateTime<Utc>
// }

// #[derive(Clone, Serialize, Deserialize)]
// pub struct SingleCharacterResult {
//     /// 投票ID（2021）
//     pub vote_id: i32,
//     /// 名字
//     pub name: String,
//     /// 排名
//     pub rank: i32,    
//     /// 票数
//     pub vote_count: i32, 
//     /// 本名加权后票数
//     pub vote_count_weighted: i32, 
//     /// 票数占比
//     pub vote_ratio: f64, 
//     /// 本名票数
//     pub vote_first_count: i32, 
//     /// 本名占比
//     pub vote_first_ratio: f64, 
//     /// 男性票数
//     pub male_count: i32, 
//     /// 男性占比
//     pub male_ratio: f64, 
//     /// 女性票数
//     pub female_count: i32, 
//     /// 女性占比
//     pub female_ratio: f64, 
//     /// 前一次排名
//     pub rank_prev: Option<i32>, 
//     /// 投票理由
//     pub reasons: Option<Reasons>, 
//     /// 票数趋势
//     pub trends: Option<Trends>,
//     /// 根据投票人物过滤的问卷
//     pub papers: Option<PaperResults>,
//     /// 同投率
//     pub cooccurrence_ratio: Option<f64> 
// }

// #[derive(Clone, Serialize, Deserialize)]
// pub struct CharacterRankResult {
//     /// 所有人物结果
//     pub characters: Vec<SingleCharacterResult>,
//     /// 使用的过滤器
//     pub filter_condtions: Option<FilterConditionsOutput>
// }


// // ------------------------------------------------
// // GQL Schemas
// // ------------------------------------------------

// #[juniper::object]
// #[graphql(description="单个人物的结果")]
// impl SingleCharacterResult {
//     /// 名字
//     pub fn name(&self) -> &String {
//         &self.name
//     }
//     /// 排名
//     pub fn rank(&self) -> &i32 {
//         &self.rank
//     }
//     /// 票数
//     pub fn vote_count(&self) -> &i32 {
//         &self.vote_count
//     }
//     /// 本名加权后票数
//     pub fn vote_count_weighted(&self) -> &i32 {
//         &self.vote_count_weighted
//     }
//     /// 票数占比
//     pub fn vote_ratio(&self) -> &f64 {
//         &self.vote_ratio
//     }
//     /// 本名票数
//     pub fn vote_first_count(&self) -> &i32 {
//         &self.vote_first_count
//     }
//     /// 本名占比
//     pub fn vote_first_ratio(&self) -> &f64 {
//         &self.vote_first_ratio
//     }
//     /// 男性票数
//     pub fn male_count(&self) -> &i32 {
//         &self.male_count
//     }
//     /// 男性占比
//     pub fn male_ratio(&self) -> &f64 {
//         &self.male_ratio
//     }
//     /// 女性票数
//     pub fn female_count(&self) -> &i32 {
//         &self.female_count
//     }
//     /// 女性占比
//     pub fn female_ratio(&self) -> &f64 {
//         &self.female_ratio
//     }
//     /// 前一次排名
//     pub fn rank_prev(&self) -> Option<i32> {
//         None
//     }
//     /// 投票理由
//     pub fn reasons(&self) -> Option<Reasons> {
//         None
//     }
//     /// 票数趋势
//     pub fn trends(&self) -> Option<Trends> {
//         None
//     }
//     /// 根据投票人物过滤的问卷
//     pub fn papers(&self) -> Option<PaperResults> {
//         None
//     }
//     /// 同投率
//     pub fn cooccurrence_ratio(&self) -> Option<f64> {
//         None
//     }
// }

// #[juniper::object]
// #[graphql(description="人物的结果")]
// impl CharacterRankResult {
//     /// 所有人物结果
//     pub fn characters(&self) -> &Vec<SingleCharacterResult> {
//         &self.characters
//     }
//     /// 使用的过滤器
//     pub fn filter_condtions(&self) -> Option<FilterConditionsOutput> {
//         None
//     }
// }

// // ------------------------------------------------
// // Root Quries
// // ------------------------------------------------

// use crate::services::*;

// // pub fn character_rank_impl(filters: Option<FilterConditions>) -> FieldResult<CharacterRankResult> {

// // }

// pub fn character_reasons_impl(name: String) -> FieldResult<Reasons> {
//     Ok(Reasons { reasons: vec![] })
// }

// pub fn single_character_result_impl(name: String, filter: Option<FilterConditions>) -> FieldResult<SingleCharacterResult> {
//     Ok(SingleCharacterResult {
//         vote_id: 2020,
//         name: "博丽灵梦".into(),
//         rank: 1,
//         vote_count: 6000,
//         vote_count_weighted: 7000,
//         vote_ratio: 0.8,
//         vote_first_count: 600,
//         vote_first_ratio: 0.1,
//         male_count: 5000,
//         male_ratio: 0.8333333333,
//         female_count: 1000,
//         female_ratio: 0.1666666666,
//         rank_prev: None,
//         reasons: None,
//         trends: None,
//         papers: None,
//         cooccurrence_ratio: None
//     })
// }

// pub fn character_rank_result_impl(filter: Option<FilterConditions>) -> FieldResult<CharacterRankResult> {
//     Ok(CharacterRankResult {
//         characters: vec![SingleCharacterResult {
//             vote_id: 2020,
//             name: "博丽灵梦".into(),
//             rank: 1,
//             vote_count: 6000,
//             vote_count_weighted: 7000,
//             vote_ratio: 0.8,
//             vote_first_count: 600,
//             vote_first_ratio: 0.1,
//             male_count: 5000,
//             male_ratio: 0.8333333333,
//             female_count: 1000,
//             female_ratio: 0.1666666666,
//             rank_prev: None,
//             reasons: None,
//             trends: None,
//             papers: None,
//             cooccurrence_ratio: None
//         }],
//         filter_condtions: match filter { Some(ref x) => Some(FilterConditionsOutput::from_input(&x)), None => None }
//     })
// }
