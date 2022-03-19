
use std::str::FromStr;

use actix_web::{App, HttpMessage, HttpRequest, HttpServer, Responder, web};
use bson::oid::ObjectId;
use jwt_simple::prelude::{Claims, ECDSAP256kPublicKeyLike};
use pvrustlib::{ServiceError, EmptyJSON};
use crate::{account_management, context::AppContext, legacy_login, models::{VoteTokenClaim}, new_login, common::SERVICE_NAME};

use super::models;

pub async fn login_email_password(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::EmailLoginInputsForExistingVoters>) -> Result<web::Json<models::LoginResults>, ServiceError> {
	let sid = request.cookie("sid").map(|f| f.to_string());
	let result = legacy_login::login_email_password(&ctx, body.email.clone(), body.password.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone(), sid).await;
	match result {
		Ok(r) => {
			let vote_token = r.generate_vote_token(ctx.vote_year, ctx.vote_start, ctx.vote_end, &ctx.key_pair)?;
			let user_token = r.generate_user_auth(&ctx.key_pair);
			return Ok(web::Json(models::LoginResults { user: r.to_fe_voter(&ctx.key_pair), vote_token: vote_token, session_token: user_token }));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn login_email(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::EmailLoginInputs>) -> Result<web::Json<models::LoginResults>, ServiceError> {
	let sid = request.cookie("sid").map(|f| f.to_string());
	let result = new_login::login_email(&ctx, body.email.clone(), body.verify_code.clone(), body.nickname.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone(), sid).await;
	match result {
		Ok(r) => {
			let vote_token = r.generate_vote_token(ctx.vote_year, ctx.vote_start, ctx.vote_end, &ctx.key_pair)?;
			let user_token = r.generate_user_auth(&ctx.key_pair);
			return Ok(web::Json(models::LoginResults { user: r.to_fe_voter(&ctx.key_pair), vote_token: vote_token, session_token: user_token }));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn login_phone(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::PhoneLoginInputs>) -> Result<web::Json<models::LoginResults>, ServiceError> {
	let sid = request.cookie("sid").map(|f| f.to_string());
	let result = new_login::login_phone(&ctx, body.phone.clone(), body.verify_code.clone(), body.nickname.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone(), sid).await;
	match result {
		Ok(r) => {
			let vote_token = r.generate_vote_token(ctx.vote_year, ctx.vote_start, ctx.vote_end, &ctx.key_pair)?;
			let user_token = r.generate_user_auth(&ctx.key_pair);
			return Ok(web::Json(models::LoginResults { user: r.to_fe_voter(&ctx.key_pair), vote_token: vote_token, session_token: user_token }));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn send_phone_verify_code(ctx: web::Data<AppContext>, body: actix_web::web::Json<models::SendPhoneVerifyCodeRequest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	println!("Sending phone code to {}", body.phone);
	let result = new_login::send_sms(&ctx, body.phone.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone()).await;
	match result {
		Ok(r) => {
			return Ok(web::Json(EmptyJSON::new()));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn send_email_verify_code(ctx: web::Data<AppContext>, body: actix_web::web::Json<models::SendEmailVerifyCodeRequest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let result = new_login::send_email(&ctx, body.email.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone()).await;
	match result {
		Ok(r) => {
			return Ok(web::Json(EmptyJSON::new()));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn update_email(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::UpdateEmailInputs>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let claim = ctx.key_pair.public_key().verify_token::<VoteTokenClaim>(&body.user_token, None).map_err(|e| ServiceError::new_jwt_error(SERVICE_NAME, Some(format!("{:?}", e))))?;
	let uid: ObjectId = ObjectId::from_str(&claim.custom.vote_id.unwrap()).unwrap();
	let result = account_management::update_email(&ctx, uid, body.email.clone(), body.verify_code.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone()).await;
	match result {
		Ok(r) => {
			return Ok(web::Json(EmptyJSON::new()));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn update_phone(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::UpdatePhoneInputs>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let claim = ctx.key_pair.public_key().verify_token::<VoteTokenClaim>(&body.user_token, None).map_err(|e| ServiceError::new_jwt_error(SERVICE_NAME, Some(format!("{:?}", e))))?;
	let uid: ObjectId = ObjectId::from_str(&claim.custom.vote_id.unwrap()).unwrap();
	let result = account_management::update_phone(&ctx, uid, body.phone.clone(), body.verify_code.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone()).await;
	match result {
		Ok(r) => {
			return Ok(web::Json(EmptyJSON::new()));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn update_nickname(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::UpdateNicknameInputs>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let claim = ctx.key_pair.public_key().verify_token::<VoteTokenClaim>(&body.user_token, None).map_err(|e| ServiceError::new_jwt_error(SERVICE_NAME, Some(format!("{:?}", e))))?;
	let uid: ObjectId = ObjectId::from_str(&claim.custom.vote_id.unwrap()).unwrap();
	let result = account_management::update_nickname(&ctx, uid, body.nickname.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone()).await;
	match result {
		Ok(r) => {
			return Ok(web::Json(EmptyJSON::new()));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}

pub async fn update_password(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::UpdatePasswordInputs>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let claim = ctx.key_pair.public_key().verify_token::<VoteTokenClaim>(&body.user_token, None).map_err(|e| ServiceError::new_jwt_error(SERVICE_NAME, Some(format!("{:?}", e))))?;
	let uid: ObjectId = ObjectId::from_str(&claim.custom.vote_id.unwrap()).unwrap();
	let result = account_management::update_password(&ctx, uid, body.old_password.clone(), body.new_password.clone(), Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone()).await;
	match result {
		Ok(r) => {
			return Ok(web::Json(EmptyJSON::new()));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}


pub async fn user_token_status(ctx: web::Data<AppContext>, body: actix_web::web::Json<models::TokenStatusInputs>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let claim = ctx.key_pair.public_key().verify_token::<VoteTokenClaim>(&body.user_token, None).map_err(|e| ServiceError::new_jwt_error(SERVICE_NAME, Some(format!("{:?}", e))))?;
	return Ok(web::Json(EmptyJSON::new()))
}

pub async fn remove_voter(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RemoveVoterRequest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let claim = ctx.key_pair.public_key().verify_token::<VoteTokenClaim>(&body.user_token, None).map_err(|e| ServiceError::new_jwt_error(SERVICE_NAME, Some(format!("{:?}", e))))?;
	let uid: ObjectId = ObjectId::from_str(&claim.custom.vote_id.unwrap()).unwrap();
	let result = account_management::remove_voter(&ctx, uid, Some(body.meta.user_ip.clone()), body.meta.additional_fingureprint.clone()).await;
	match result {
		Ok(r) => {
			return Ok(web::Json(EmptyJSON::new()));
		},
		Err(e) => {
			return Err(ServiceError::from_dyn_error(SERVICE_NAME, e));
		},
	}
}
