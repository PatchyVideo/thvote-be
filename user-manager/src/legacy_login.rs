
use crate::{context::AppContext, log, models::{ActivityLogEntry, Voter}, common::{SERVICE_NAME, rate_limit}};
use argon2::Config;
use mongodb::bson::{doc};
use bson::DateTime;
use pvrustlib::ServiceError;
use rand::{RngCore, rngs::OsRng};


pub async fn login_email_password(ctx: &AppContext, email: String, password: String, ip: Option<String>, additional_fingerprint: Option<String>, sid: Option<String>) -> Result<Voter, Box<dyn std::error::Error>> {
	let mut redis_conn = ctx.redis_client.get_async_connection().await?;
	if let Some(voter) = ctx.voters_coll.find_one(doc! { "email": email.clone() }, None).await? {
		rate_limit(&voter._id.unwrap(), &mut redis_conn).await?;
		if let Some(password_hashed) = voter.password_hashed.as_ref() {
			if let Some(salt) = voter.salt.as_ref() {
				let pwrt = format!("{}{}", password, salt);
				if !bcrypt::verify(pwrt, password_hashed).ok().unwrap_or(false) {
					return Err(Box::new(ServiceError::new_error_kind(SERVICE_NAME, "INCORRECT_PASSWORD")));
				} else {
					// legacy bcrypt verified
					// upgrade to argon2
					let mut salt = [0u8; 16];
					OsRng.fill_bytes(&mut salt);
					let new_password_hashed = argon2::hash_encoded(password.as_bytes(), &salt, &Config::default())?;
					let mut voter = voter.clone();
					voter.salt = None;
					voter.password_hashed = Some(new_password_hashed.clone());
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
					ctx.voters_coll.replace_one(doc! { "email": email.clone() }, voter.clone(), None).await?;
					log(ctx, ActivityLogEntry::VoterLogin {
						created_at: DateTime::now(),
						uid: voter._id.as_ref().unwrap().clone(),
						phone: None,
						email: None,
						requester_ip: ip.clone(),
						requester_additional_fingerprint: additional_fingerprint.clone()
					}).await;
					return Ok(voter);
				}
			}
			if argon2::verify_encoded(password_hashed, password.as_bytes())? {
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
				}
				log(ctx, ActivityLogEntry::VoterLogin {
					created_at: DateTime::now(),
					uid: voter._id.as_ref().unwrap().clone(),
					phone: None,
					email: None,
					requester_ip: ip.clone(),
					requester_additional_fingerprint: additional_fingerprint.clone()
				}).await;
				Ok(voter)
			} else {
				return Err(Box::new(ServiceError::new_error_kind(SERVICE_NAME, "INCORRECT_PASSWORD")));
			}
		} else {
			return Err(ServiceError::new_error_kind(SERVICE_NAME, "LOGIN_METHOD_NOT_SUPPORTED").into());
		}
	} else {
		if let Some(ip) = ip {
			rate_limit(&ip, &mut redis_conn).await?;
		}
		return Err(ServiceError::new_not_found(SERVICE_NAME, None).into());
	}
}
