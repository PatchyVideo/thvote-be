
use juniper::graphql_value;

use juniper::FieldResult;
use pvrustlib::EmptyJSON;
use pvrustlib::json_request_gateway;

use crate::common::SERVICE_NAME;
use crate::context::Context;
use crate::submit_handler::VotingStatus;

use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct UserEventMeta {
	pub user_ip: String,
	pub additional_fingureprint: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendPhoneVerifyCodeRequest {
	pub phone: String,
    pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendEmailVerifyCodeRequest {
	pub email: String,
    pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmailLoginInputsForExistingVoters {
	pub email: String,
	pub password: String,
	pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmailLoginInputs {
	pub email: String,
	pub nickname: Option<String>,
	pub verify_code: String,
	pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PhoneLoginInputs {
	pub phone: String,
	pub nickname: Option<String>,
	pub verify_code: String,
	pub meta: UserEventMeta
}

// ------------------------------------------------
// GQL Schemas
// ------------------------------------------------

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Voter")]
pub struct Voter {
	pub username: Option<String>,
	pub pfp: Option<String>,
	pub password: bool,
	pub phone: Option<String>,
	pub email: Option<String>,
	pub thbwiki: bool,
	pub patchyvideo: bool,
	pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Login results")]
pub struct LoginResults {
	/// 用户
	pub user: Voter,
	/// 投票token
	pub vote_token: String,
	/// 用户登录token
	pub session_token: String
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;

/// 老用户使用email帐号登录
pub async fn login_email_password(context: &Context, email: String, password: String) -> FieldResult<LoginResults> {
	let email = email.to_ascii_lowercase();
	let submit_json = EmailLoginInputsForExistingVoters {
		email: email,
		password: password,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	Ok(json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/login-email-password", USER_MANAGER), submit_json).await?)
}

/// 新用户使用email帐号登录
pub async fn login_email(context: &Context,  email: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
	let email = email.to_ascii_lowercase();
	let submit_json = EmailLoginInputs {
		email: email,
		verify_code: verify_code,
		nickname: nickname,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	Ok(json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/login-email", USER_MANAGER), submit_json).await?)
}
/// 向邮箱发送验证码
pub async fn request_email_code(context: &Context, email: String) -> FieldResult<bool> {
	let email = email.to_ascii_lowercase();
	let submit_json = SendEmailVerifyCodeRequest {
		email: email,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	let _tmp: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/send-email-code", USER_MANAGER), submit_json).await?;
	Ok(true)
}

/// 使用手机帐号登录
pub async fn login_phone(context: &Context, phone: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
	let submit_json = PhoneLoginInputs {
		phone: phone,
		verify_code: verify_code,
		nickname: nickname,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	Ok(json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/login-phone", USER_MANAGER), submit_json).await?)
}
/// 向手机发送验证码
pub async fn request_phone_code(context: &Context, phone: String) -> FieldResult<bool> {
	let submit_json = SendPhoneVerifyCodeRequest {
		phone: phone,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	let _tmp: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/send-sms-code", USER_MANAGER), submit_json).await?;
	Ok(true)
}


#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateEmailInputs {
	pub user_token: String,
    pub email: String,
    pub verify_code: String,
    pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdatePhoneInputs {
	pub user_token: String,
    pub phone: String,
    pub verify_code: String,
    pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateNicknameInputs {
	pub user_token: String,
    pub nickname: String,
    pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdatePasswordInputs {
	pub user_token: String,
    pub old_password: Option<String>,
    pub new_password: String,
    pub meta: UserEventMeta
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatusInputs {
	pub user_token: String,
	pub vote_token: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenStatusOutput {
	pub status: String,
	pub voting_status: Option<VotingStatus>,
	pub papers_json: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RemoveVoterRequest {
	pub user_token: String,
    pub old_password: Option<String>,
    pub meta: UserEventMeta
}

pub async fn update_email(context: &Context, user_token: String, email: String, verify_code: String) -> FieldResult<bool> {
	let email = email.to_ascii_lowercase();
	let submit_json = UpdateEmailInputs {
		email: email,
		verify_code: verify_code,
		user_token: user_token,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	let t: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/update-email", USER_MANAGER), submit_json).await?;
	Ok(true)
}

pub async fn update_phone(context: &Context, user_token: String, phone: String, verify_code: String) -> FieldResult<bool> {
	let submit_json = UpdatePhoneInputs {
		phone: phone,
		verify_code: verify_code,
		user_token: user_token,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	let t: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/update-phone", USER_MANAGER), submit_json).await?;
	Ok(true)
}

pub async fn update_nickname(context: &Context, user_token: String, new_nickname: String) -> FieldResult<bool> {
	let submit_json = UpdateNicknameInputs {
		nickname: new_nickname,
		user_token: user_token,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	let t: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/update-nickname", USER_MANAGER), submit_json).await?;
	Ok(true)
}

pub async fn update_password(context: &Context, user_token: String, old_password: Option<String>, new_password: String) -> FieldResult<bool> {
	let submit_json = UpdatePasswordInputs {
		old_password: old_password,
		new_password: new_password,
		user_token: user_token,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	let t: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/update-password", USER_MANAGER), submit_json).await?;
	Ok(true)
}

pub async fn user_token_status(user_token: String, vote_token: Option<String>) -> FieldResult<bool> {
	let submit_json = TokenStatusInputs {
		user_token: user_token,
		vote_token: vote_token
	};
	let t: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/user-token-status", USER_MANAGER), submit_json).await?;
	Ok(true)
}

pub async fn remove_voter(context: &Context, user_token: String, old_password: Option<String>) -> FieldResult<bool> {
	let submit_json = RemoveVoterRequest {
		old_password: old_password,
		user_token: user_token,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	let t: EmptyJSON = json_request_gateway(SERVICE_NAME, &format!("http://{}/v1/remove-voter", USER_MANAGER), submit_json).await?;
	Ok(true)
}

