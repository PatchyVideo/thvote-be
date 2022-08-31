use juniper::FieldResult;
use juniper::GraphQLSubscriptionValue;
use juniper::RootNode;

use chrono::{DateTime, Utc};

use crate::result_query::CPRanking;
use crate::result_query::CharacterOrMusicRanking;
use crate::result_query::Reasons;
use crate::result_query::Trends;
use crate::submit_handler::CPSubmitGQL;
use crate::submit_handler::CPSubmitRestQuery;
use crate::submit_handler::CharacterSubmitGQL;
use crate::submit_handler::CharacterSubmitRestQuery;
use crate::submit_handler::DojinSubmitGQL;
use crate::submit_handler::DojinSubmitRestQuery;
use crate::submit_handler::MusicSubmitGQL;
use crate::submit_handler::MusicSubmitRestQuery;
use crate::submit_handler::PaperSubmitGQL;
use crate::submit_handler::PaperSubmitRestQuery;
use crate::submit_handler::WorkSubmitGQL;
use crate::user_manager::EmailLoginInputs;
use crate::user_manager::EmailLoginInputsForExistingVoters;
use crate::user_manager::LoginResults;
use crate::user_manager::PhoneLoginInputs;

use crate::{user_manager, submit_handler, vote_data, result_query};

use super::context::Context;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

	// ------------------------------------------------
	//     result_query
	// ------------------------------------------------

	// /// 人物投票理由
	// fn character_reasons(name: String) -> FieldResult<Reasons> {
	// 	result_query::character_reasons_impl(name)
	// }

	// /// 人物投票结果
	// fn character_rank_result(filter: Option<FilterConditions>) -> FieldResult<CharacterRankResult> {
	// 	result_query::character_rank_result_impl(filter)
	// }

	// /// 人物投票理由
	// fn single_character_result(name: String, filter: Option<FilterConditions>) -> FieldResult<SingleCharacterResult> {
	// 	result_query::single_character_result_impl(name, filter)
	// }

	
	// ------------------------------------------------
	//     vote data
	// ------------------------------------------------
	async fn listVotableCharacters(context: &Context) -> FieldResult<vote_data::VotableCharacters> {
		vote_data::listVotableCharacters_impl(context).await
	}
	async fn listVotableWorks(context: &Context) -> FieldResult<vote_data::VotableWorks> {
		vote_data::listVotableWorks_impl(context).await
	}
	async fn listVotableMusics(context: &Context) -> FieldResult<vote_data::VotableMusics> {
		vote_data::listVotableMusics_impl(context).await

	}

	// ------------------------------------------------
	//     user management
	// ------------------------------------------------
	async fn userTokenStatus(context: &Context, user_token: String, vote_token: Option<String>) -> FieldResult<bool> {
		user_manager::user_token_status(user_token, vote_token).await
	}

	// ------------------------------------------------
	//     submit_handler
	// ------------------------------------------------
	
	/// Get Character
	async fn getSubmitCharacterVote(context: &Context, vote_token: String) -> FieldResult<CharacterSubmitRestQuery> {
		submit_handler::getSubmitCharacterVote_impl(context, vote_token).await
	}

	/// Get Music
	async fn getSubmitMusicVote(context: &Context, vote_token: String) -> FieldResult<MusicSubmitRestQuery> {
		submit_handler::getSubmitMusicVote_impl(context, vote_token).await
	}

	/// Get CP
	async fn getSubmitCPVote(context: &Context, vote_token: String) -> FieldResult<CPSubmitRestQuery> {
		submit_handler::getSubmitCPVote_impl(context, vote_token).await
	}

	/// Get Paper
	async fn getSubmitPaperVote(context: &Context, vote_token: String) -> FieldResult<PaperSubmitRestQuery> {
		submit_handler::getSubmitPaperVote_impl(context, vote_token).await
	}

	/// Get Dojin
	async fn getSubmitDojinVote(context: &Context, vote_token: String) -> FieldResult<DojinSubmitRestQuery> {
		submit_handler::getSubmitDojinVote_impl(context, vote_token).await
	}
	
	// ------------------------------------------------
	//     result_query
	// ------------------------------------------------
	async fn queryCharacterRanking(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<CharacterOrMusicRanking> {
		result_query::queryCharacterRanking_impl(context, query, vote_start, vote_year).await
	}
	async fn queryCharacterReasons(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<Reasons> {
		result_query::queryCharacterReasons_impl(context, query, vote_start, vote_year, rank).await
	}
	async fn queryCharacterTrend(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, name: String) -> FieldResult<Trends> {
		result_query::queryCharacterTrend_impl(context, query, vote_start, vote_year, name).await
	}
	async fn queryMusicRanking(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<CharacterOrMusicRanking> {
		result_query::queryMusicRanking_impl(context, query, vote_start, vote_year).await
	}
	async fn queryMusicReasons(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<Reasons> {
		result_query::queryMusicReasons_impl(context, query, vote_start, vote_year, rank).await
	}
	async fn queryMusicTrend(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, name: String) -> FieldResult<Trends> {
		result_query::queryMusicTrend_impl(context, query, vote_start, vote_year, name).await
	}
	async fn queryCPRanking(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32) -> FieldResult<CPRanking> {
		result_query::queryCPRanking_impl(context, query, vote_start, vote_year).await
	}
	async fn queryCPReasons(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, rank: i32) -> FieldResult<Reasons> {
		result_query::queryCPReasons_impl(context, query, vote_start, vote_year, rank).await
	}
	async fn queryCPTrend(context: &Context, query: Option<String>, vote_start: DateTime<Utc>, vote_year: i32, name: String) -> FieldResult<Trends> {
		result_query::queryCPTrend_impl(context, query, vote_start, vote_year, name).await
	}
}


pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

	// ------------------------------------------------
	//     user_manager
	// ------------------------------------------------

	/// 老用户使用email帐号登录
	async fn login_email_password(context: &Context, email: String, password: String) -> FieldResult<LoginResults> {
		user_manager::login_email_password(context, email, password).await
	}

	/// 新用户使用email帐号登录
	async fn login_email(context: &Context,  email: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
		user_manager::login_email(context, email, nickname, verify_code).await
	}
	/// 向邮箱发送验证码
	async fn request_email_code(context: &Context, email: String) -> FieldResult<bool> {
		user_manager::request_email_code(context, email).await
	}

	/// 使用手机帐号登录
	async fn login_phone(context: &Context, phone: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
		user_manager::login_phone(context, phone, nickname, verify_code).await
	}
	/// 向手机发送验证码
	async fn request_phone_code(context: &Context, phone: String) -> FieldResult<bool> {
		user_manager::request_phone_code(context, phone).await
	}

	/// 更新邮箱
	async fn update_email(context: &Context, user_token: String, email: String, verify_code: String) -> FieldResult<bool> {
		user_manager::update_email(context, user_token, email, verify_code).await
	}

	/// 更新手机
	async fn update_phone(context: &Context, user_token: String, phone: String, verify_code: String) -> FieldResult<bool> {
		user_manager::update_phone(context, user_token, phone, verify_code).await
	}

	/// 更新昵称
	async fn update_nickname(context: &Context, user_token: String, new_nickname: String) -> FieldResult<bool> {
		user_manager::update_nickname(context, user_token, new_nickname).await
	}

	/// 更新密码
	async fn update_password(context: &Context, user_token: String, old_password: Option<String>, new_password: String) -> FieldResult<bool> {
		user_manager::update_password(context, user_token, old_password, new_password).await
	}

	/// 账号注销
	async fn remove_voter(context: &Context, user_token: String, old_password: Option<String>) -> FieldResult<bool> {
		user_manager::remove_voter(context, user_token, old_password).await
	}

	// ------------------------------------------------
	//     submit_handler
	// ------------------------------------------------

	/// Character
	async fn submitCharacterVote(context: &Context, content: CharacterSubmitGQL) -> FieldResult<bool> {
		submit_handler::submitCharacterVote_impl(context, &content).await
	}

	/// music
	async fn submitMusicVote(context: &Context, content: MusicSubmitGQL) -> FieldResult<bool> {
	   submit_handler::submitMusicVote_impl(context, &content).await
	}
	
	/// CP
	async fn submitCPVote(context: &Context, content: CPSubmitGQL) -> FieldResult<bool> {
		submit_handler::submitCPVote_impl(context, &content).await
	}

	/// paper
	async fn submitPaperVote(context: &Context, content: PaperSubmitGQL) -> FieldResult<bool> {
		submit_handler::submitPaperVote_impl(context, &content).await
	}

	/// dojins
	async fn submitDojin(context: &Context, content: DojinSubmitGQL) -> FieldResult<bool> {
		submit_handler::submitDojinVote_impl(context, &content).await
	}
}

pub struct Subscription;

#[juniper::graphql_object(Context = Context)]
impl Subscription {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

}

impl GraphQLSubscriptionValue for Subscription {
	
}

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn create_schema() -> Schema {
	Schema::new(Query {}, Mutation {}, Subscription {})
}
