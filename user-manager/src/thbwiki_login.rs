use crate::{context::{AppContext, LoginSession}, models::{Voter}};
use argon2::Config;
use mongodb::bson::{doc};
use chrono::Utc;
use chrono::prelude::*;
use rand::{Rng, rngs::OsRng, RngCore};

// pub async fn redirect_callback(ctx: &AppContext, uid: String, email: Option<String>, nickname: Option<String>, signup_ip: Option<String>) -> Result<Voter, Box<dyn std::error::Error>> {
// 	if email.is_none() {
// 		// Email not verified by THBWiki
// 		let sess = LoginSession {
// 			thbwiki_uid: Some(uid),
// 			qq_openid: None,
// 			signup_ip: signup_ip
// 		};
// 		let sid = ctx.create_login_session(sess);
// 		return Err(Box::new(ServiceError::RedirectToSignup{ sid: sid, nickname: nickname }))
// 	}
// 	let email = email.unwrap();
// 	if let Some(voter) = ctx.voters_coll.find_one(doc! { "email": email.clone() }, None).await? {
// 		let mut voter = voter.clone();
// 		voter.thbwiki_uid = Some(uid);
// 		ctx.voters_coll.replace_one(doc! { "email": email.clone() }, voter.clone(), None).await?;
// 		Ok(voter)
// 	} else {
// 		let mut voter = Voter {
// 			_id: None,
// 			email: Some(email),
// 			email_verified: true,
// 			password_hashed: None,
// 			salt: None,
// 			created_at: bson::DateTime(Utc::now()),
// 			nickname: nickname,
// 			signup_ip: signup_ip,
// 			qq_openid: None,
// 			thbwiki_uid: Some(uid),
// 			phone: None,
// 			pfp: None,
// 			phone_verified: false,
// 			removed: None
// 		};
// 		let iid = ctx.voters_coll.insert_one(voter.clone(), None).await?;
// 		voter._id = Some(iid.inserted_id.as_object_id().unwrap().clone());
// 		Ok(voter)
// 	}
// }
