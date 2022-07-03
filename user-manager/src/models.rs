
use std::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use chrono::{Utc};
use jwt_simple::prelude::{Claims, Duration, ECDSAP256kKeyPairLike, ES256kKeyPair, UnixTimeStamp};
use pvrustlib::ServiceError;
use serde::{Serialize, Deserialize};
use bson::{DateTime, oid::ObjectId};

use crate::{context::LoginSession, common::SERVICE_NAME};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteTokenClaim {
	pub vote_id: Option<String>
}


#[derive(Clone, Serialize, Deserialize)]
/// 给前端的投票人
pub struct VoterFE {
	pub username: Option<String>,
	pub pfp: Option<String>,
	pub password: bool,
	pub phone: Option<String>,
	pub email: Option<String>,
	pub thbwiki: bool,
	pub patchyvideo: bool,
	pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 投票人
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Voter {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub _id: Option<ObjectId>,
	pub phone: Option<String>,
	pub phone_verified: bool,
	pub email: Option<String>,
	pub email_verified: bool,
	/// Not required if THBWiki login or SMS login is used
	pub password_hashed: Option<String>,
	/// Used only in legacy login
	pub salt: Option<String>,
	/// 新版投票用户创建日期
	pub created_at: DateTime,
	pub nickname: Option<String>,
	pub signup_ip: Option<String>,
	pub qq_openid: Option<String>,
	pub pfp: Option<String>,
	pub thbwiki_uid: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub removed: Option<bool>
}

impl Voter {
	/// Generate a unqiue id connectted to voter for a given year
	pub fn generate_vote_id(&self, vote_year: u32) -> Result<String, ServiceError> {
		if self.phone_verified || self.email_verified {
			let id = self._id.as_ref().unwrap().clone().to_string();
			return Ok(format!("thvote-{}-{}", vote_year, id));
		}
		return Err(ServiceError::new_error_kind(SERVICE_NAME, "USER_UNVERIFIED"));
	}
	/// Generate a signed JWT token for voting with
	/// 1. vote-id
	/// 2. valid since
	/// 3. valid until
	/// 4. scope (vote or login)
	pub fn generate_vote_token(&self, vote_year: u32, vote_start: chrono::DateTime<chrono::Utc>, vote_end: chrono::DateTime<chrono::Utc>, key: &ES256kKeyPair) -> Result<String, ServiceError> {
		let addtional_info = VoteTokenClaim {
			vote_id: Some(self.generate_vote_id(vote_year)?)
		};
		let diff = vote_end - vote_start;
		let claims = Claims::with_custom_claims_given_valid_period(
			addtional_info, 
			UnixTimeStamp::new(vote_start.timestamp() as u64, 0), 
			Duration::from_secs(diff.num_seconds() as _)
		)
		.with_audience("vote");
		// let claims = Claims::with_custom_claims(addtional_info, Duration::from_hours(7 * 24))
		// 	.with_audience("vote");
		Ok(key.sign(claims).unwrap())
	}
	/// Generate a signed JWT token for user space with
	/// 1. valid until
	/// 2. scope (vote or login)
	pub fn generate_user_auth(&self, key: &ES256kKeyPair) -> String {
		let addtional_info = VoteTokenClaim {
			vote_id: Some(self._id.as_ref().unwrap().clone().to_string())
		};
		let claims = Claims::with_custom_claims(addtional_info, Duration::from_hours(7 * 24))
			.with_audience("userspace");
		key.sign(claims).unwrap()
	}
	pub fn to_fe_voter(&self, key: &ES256kKeyPair) -> VoterFE {
		VoterFE {
			username: self.nickname.clone(),
			pfp: self.pfp.clone(),
			password: self.password_hashed.is_some(),
			phone: self.phone.clone(),
			email: self.email.clone(),
			thbwiki: false,
			patchyvideo: false,
			created_at: self.created_at.to_chrono()
		}
	}
}

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

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenStatusInputs {
	pub user_token: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PhoneLoginInputs {
    pub phone: String,
    pub nickname: Option<String>,
    pub verify_code: String,
    pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginResults {
	/// 用户
	pub user: VoterFE,
	/// 投票token
	pub vote_token: String,
	/// 用户登录token
	pub session_token: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityLogEntry {
	SendEmail {
		created_at: DateTime,
		target_email: String,
		code: String,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	},
	SendSMS {
		created_at: DateTime,
		target_phone: String,
		code: String,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	},
	VoterCreation {
		created_at: DateTime,
		uid: ObjectId,
		email: Option<String>,
		phone: Option<String>,
		nickname: Option<String>,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	},
	VoterLogin {
		created_at: DateTime,
		uid: ObjectId,
		email: Option<String>,
		phone: Option<String>,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	},
	UpdateEmail {
		created_at: DateTime,
		uid: ObjectId,
		old_email: Option<String>,
		new_email: String,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	},
	UpdatePhone {
		created_at: DateTime,
		uid: ObjectId,
		old_phone: Option<String>,
		new_phone: String,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	},
	UpdateNickname {
		created_at: DateTime,
		uid: ObjectId,
		old_nickname: Option<String>,
		new_nickname: String,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>,
	},
	UpdatePassword {
		created_at: DateTime,
		uid: ObjectId,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	},
	RemoveVoter {
		created_at: DateTime,
		uid: ObjectId,
		requester_ip: Option<String>,
		requester_additional_fingerprint: Option<String>
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RemoveVoterRequest {
	pub user_token: String,
    pub old_password: Option<String>,
    pub meta: UserEventMeta
}
