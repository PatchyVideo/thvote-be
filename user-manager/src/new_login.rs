use std::{fmt::format, ops::RangeInclusive};

use crate::{context::AppContext, models::{ActivityLogEntry, Voter}, common::{SERVICE_NAME, rate_limit}};
use argon2::Config;
use bson::DateTime;
use mongodb::bson::{doc};
use chrono::Utc;
use chrono::prelude::*;
use pvrustlib::{ServiceError, EmptyJSON, json_request};
use rand::{Rng, RngCore, distributions::uniform::SampleRange, rngs::OsRng};
use rand::distributions::{Distribution, Uniform};
use redis::AsyncCommands;

use crate::log;

const SMS_INTERVAL: usize = 120;
const EMAIL_INTERVAL: usize = 120;

pub async fn check_email_availability(ctx: &AppContext, email: String) -> Result<bool, Box<dyn std::error::Error>> {
	Ok(ctx.voters_coll.find_one(doc! { "email": email }, None).await?.is_none())
}

pub async fn signup_email(ctx: &AppContext, email: String, verify_code: String, nickname: Option<String>, ip: Option<String>, additional_fingerprint: Option<String>, sid: Option<String>) -> Result<Voter, Box<dyn std::error::Error>> {
	if let None = ctx.voters_coll.find_one(doc! { "email": email.clone() }, None).await? {
		let mut voter = Voter {
			_id: None,
			email: Some(email.clone()),
			email_verified: true,
			phone: None,
			phone_verified: false,
			password_hashed: None,
			salt: None,
			created_at: DateTime::now(),
			nickname: nickname.clone(),
			signup_ip: ip.clone(),
			qq_openid: None,
			pfp: None,
			thbwiki_uid: None,
			removed: None
		};
		if let Some(sid) = sid {
			if let Some(sess) = ctx.get_login_session(&sid).await {
				if let Some(thbwiki_uid) = sess.thbwiki_uid {
					voter.thbwiki_uid = Some(thbwiki_uid);
				}
				if let Some(qq_openid) = sess.qq_openid {
					voter.qq_openid = Some(qq_openid);
				}
			}
		}
		let iid = ctx.voters_coll.insert_one(voter.clone(), None).await?;
		println!("{}", iid.inserted_id);
		voter._id = Some(iid.inserted_id.as_object_id().unwrap().clone());
		log(ctx, ActivityLogEntry::VoterCreation {
			created_at: DateTime::now(),
			uid: voter._id.as_ref().unwrap().clone(),
			nickname: nickname,
			phone: None,
			email: Some(email),
			requester_ip: ip.clone(),
			requester_additional_fingerprint: additional_fingerprint.clone()
		}).await;
		Ok(voter)
	} else {
		return Err(ServiceError::new_error_kind(SERVICE_NAME, "USER_ALREADY_EXIST").into());
	}
}

pub async fn login_email(ctx: &AppContext, email: String, verify_code: String, nickname: Option<String>, ip: Option<String>, additional_fingerprint: Option<String>, sid: Option<String>) -> Result<Voter, Box<dyn std::error::Error>> {
	let id = format!("email-verify-{}", email);
	let mut conn = ctx.redis_client.get_async_connection().await?;
	let expected_code: Option<String> = conn.get(&id).await?;
	rate_limit(&email, &mut conn).await?;
	if let None = expected_code {
		return Err(ServiceError::new_error_kind(SERVICE_NAME, "INCORRECT_VERIFY_CODE").into());
	}
	let expected_code = expected_code.unwrap();
	if expected_code != verify_code {
		return Err(ServiceError::new_error_kind(SERVICE_NAME, "INCORRECT_VERIFY_CODE").into());
	}
	conn.del(id).await?;
	if let Some(voter) = ctx.voters_coll.find_one(doc! { "email": email.clone() }, None).await? {
		let mut voter = voter.clone();
		if let Some(sid) = sid {
			if let Some(sess) = ctx.get_login_session(&sid).await {
				if let Some(thbwiki_uid) = sess.thbwiki_uid {
					voter.thbwiki_uid = Some(thbwiki_uid);
				}
				if let Some(qq_openid) = sess.qq_openid {
					voter.qq_openid = Some(qq_openid);
				}
				ctx.voters_coll.replace_one(doc! { "email": email.clone() }, voter.clone(), None).await?;
			}
		};
		log(ctx, ActivityLogEntry::VoterLogin {
			created_at: DateTime::now(),
			uid: voter._id.as_ref().unwrap().clone(),
			phone: None,
			email: Some(email),
			requester_ip: ip.clone(),
			requester_additional_fingerprint: additional_fingerprint.clone()
		}).await;
		Ok(voter)
	} else {
		signup_email(ctx, email, verify_code, nickname, ip, additional_fingerprint, sid).await
	}
}

pub async fn send_email(ctx: &AppContext, email: String, ip: Option<String>, additional_fingerprint: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
	let id = format!("email-verify-{}", email);
	let id_guard = format!("email-verify-guard-{}", email);
	let mut redis_conn = ctx.redis_client.get_async_connection().await?;
	// check if 1 minutes has passed since last SMS to the same email is sent
	let guard: Option<String> = redis_conn.get(id_guard.clone()).await?;
	if let Some(guard) = guard {
		if guard == "guard" {
			return Err(ServiceError::new_error_kind(SERVICE_NAME, "REQUEST_TOO_FREQUENT").into());
		}
	}
	// generate 6 digits code
	let code_u32 = OsRng.gen_range(RangeInclusive::new(0u32,  999999u32));
	let code = format!("{:06}", code_u32);
	// store in redis, expires in 1 hour
	redis_conn.set_ex(id, code.clone(), 3600).await?;
	// store guard in redis, expires in EMAIL_INTERVAL
	redis_conn.set_ex(id_guard, "guard", EMAIL_INTERVAL).await?;
	// invoke Email send service
	println!(" -- [Email] Code = {}", code);
	let req = crate::email_service::EmailRequest {
		code: code.clone(),
		email: email.clone()
	};

	let resp: EmptyJSON = json_request(SERVICE_NAME, &format!("{}/v1/vote-code", crate::comm::SERVICE_EMAIL_ADDRESS), req).await?;

	// log if succeed
	log(ctx, ActivityLogEntry::SendEmail {
		created_at: DateTime::now(),
		target_email: email,
		code: code,
		requester_ip: ip,
		requester_additional_fingerprint: additional_fingerprint
	}).await;
	Ok(())
}

pub async fn check_phone_availability(ctx: &AppContext, phone: String) -> Result<bool, Box<dyn std::error::Error>> {
	Ok(ctx.voters_coll.find_one(doc! { "phone": phone }, None).await?.is_none())
}

pub async fn signup_phone(ctx: &AppContext, phone: String, verify_code: String, nickname: Option<String>, ip: Option<String>, additional_fingerprint: Option<String>, sid: Option<String>) -> Result<Voter, Box<dyn std::error::Error>> {
	if let None = ctx.voters_coll.find_one(doc! { "phone": phone.clone() }, None).await? {
		let mut voter = Voter {
			_id: None,
			email: None,
			email_verified: false,
			phone: Some(phone.clone()),
			phone_verified: true,
			password_hashed: None,
			salt: None,
			created_at: DateTime::now(),
			nickname: nickname.clone(),
			signup_ip: ip.clone(),
			qq_openid: None,
			pfp: None,
			thbwiki_uid: None,
			removed: None
		};
		if let Some(sid) = sid {
			if let Some(sess) = ctx.get_login_session(&sid).await {
				if let Some(thbwiki_uid) = sess.thbwiki_uid {
					voter.thbwiki_uid = Some(thbwiki_uid);
				}
				if let Some(qq_openid) = sess.qq_openid {
					voter.qq_openid = Some(qq_openid);
				}
			}
		}
		let iid = ctx.voters_coll.insert_one(voter.clone(), None).await?;
		voter._id = Some(iid.inserted_id.as_object_id().unwrap().clone());
		log(ctx, ActivityLogEntry::VoterCreation {
			created_at: DateTime::now(),
			uid: voter._id.as_ref().unwrap().clone(),
			nickname: nickname,
			phone: Some(phone),
			email: None,
			requester_ip: ip.clone(),
			requester_additional_fingerprint: additional_fingerprint.clone()
		}).await;
		Ok(voter)
	} else {
		return Err(ServiceError::new_error_kind(SERVICE_NAME, "USER_ALREADY_EXIST").into());
	}
}

pub async fn send_sms(ctx: &AppContext, phone: String, ip: Option<String>, additional_fingerprint: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
	let id = format!("phone-verify-{}", phone);
	let id_guard = format!("phone-verify-guard-{}", phone);
	let mut redis_conn = ctx.redis_client.get_async_connection().await?;
	// check if 1 minute has passed since last SMS to the same phone is sent
	let guard: Option<String> = redis_conn.get(id_guard.clone()).await?;
	if let Some(guard) = guard {
		if guard == "guard" {
			return Err(ServiceError::new_error_kind(SERVICE_NAME, "REQUEST_TOO_FREQUENT").into());
		}
	}
	// generate 6 digits code
	let code_u32 = OsRng.gen_range(RangeInclusive::new(0u32,  999999u32));
	let code = format!("{:06}", code_u32);
	// store in redis, expires in 1 hour
	redis_conn.set_ex(id, code.clone(), 3600).await?;
	// store guard in redis, expires in SMS_INTERVAL
	redis_conn.set_ex(id_guard, "guard", SMS_INTERVAL).await?;
	// invoke SMS send service
	println!(" -- [SMS] Code = {}", code);
	let req = crate::sms_service::SMSRequest {
		code: code.clone(),
		mobile: phone.clone()
	};

	let resp: EmptyJSON = json_request(SERVICE_NAME, &format!("{}/v1/vote-code", crate::comm::SERVICE_SMS_ADDRESS), req).await?;
	// log if succeed
	log(ctx, ActivityLogEntry::SendSMS {
		created_at: DateTime::now(),
		target_phone: phone,
		code: code,
		requester_ip: ip,
		requester_additional_fingerprint: additional_fingerprint
	}).await;
	Ok(())
}

pub async fn login_phone(ctx: &AppContext, phone: String, verify_code: String, nickname: Option<String>, ip: Option<String>, additional_fingerprint: Option<String>, sid: Option<String>) -> Result<Voter, Box<dyn std::error::Error>> {
	let id = format!("phone-verify-{}", phone);
	let mut conn = ctx.redis_client.get_async_connection().await?;
	rate_limit(&phone, &mut conn).await?;
	let expected_code: Option<String> = conn.get(&id).await?;
	if let None = expected_code {
		return Err(ServiceError::new_error_kind(SERVICE_NAME, "INCORRECT_VERIFY_CODE").into());
	}
	let expected_code = expected_code.unwrap();
	if expected_code != verify_code {
		println!("{}", expected_code);
		return Err(ServiceError::new_error_kind(SERVICE_NAME, "INCORRECT_VERIFY_CODE").into());
	}
	conn.del(id).await?;
	if let Some(voter) = ctx.voters_coll.find_one(doc! { "phone": phone.clone() }, None).await? {
		let mut voter = voter.clone();
		if let Some(sid) = sid {
			if let Some(sess) = ctx.get_login_session(&sid).await {
				if let Some(thbwiki_uid) = sess.thbwiki_uid {
					voter.thbwiki_uid = Some(thbwiki_uid);
				}
				if let Some(qq_openid) = sess.qq_openid {
					voter.qq_openid = Some(qq_openid);
				}
				ctx.voters_coll.replace_one(doc! { "phone": phone.clone() }, voter.clone(), None).await?;
			}
		};
		log(ctx, ActivityLogEntry::VoterLogin {
			created_at: DateTime::now(),
			uid: voter._id.as_ref().unwrap().clone(),
			phone: Some(phone),
			email: None,
			requester_ip: ip.clone(),
			requester_additional_fingerprint: additional_fingerprint.clone()
		}).await;
		Ok(voter)
	} else {
		signup_phone(ctx, phone, verify_code, nickname, ip, additional_fingerprint, sid).await
	}
}
