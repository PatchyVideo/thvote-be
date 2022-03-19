use crate::{context::{AppContext, LoginSession}, models::{Voter}};
use argon2::Config;
use mongodb::bson::{doc};
use chrono::Utc;
use chrono::prelude::*;
use rand::{Rng, rngs::OsRng, RngCore};

// pub async fn redirect_callback(ctx: &AppContext, openid: String, nickname: Option<String>, signup_ip: Option<String>) -> Result<Voter, Box<dyn std::error::Error>> {
// 	let sess = LoginSession {
// 		thbwiki_uid: None,
// 		qq_openid: Some(openid),
// 		signup_ip: signup_ip
// 	};
// 	let sid = ctx.create_login_session(sess);
// 	return Err(Box::new(ServiceError::RedirectToSignup{ sid: sid, nickname: nickname }))
// }
