#![allow(non_snake_case)]
extern crate juniper;

use std::io::{self, Read};
use std::sync::Arc;


use actix_cors::Cors;
use actix_web::http;
use actix_web::{App, Error, HttpMessage, HttpResponse, HttpServer, cookie, middleware, web};
use chrono::Utc;
use context::Context;
use juniper_actix::{
	graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use jwt_simple::prelude::{ES256kKeyPair, ES256kPublicKey};
use once_cell::sync::OnceCell;
use submit_handler::{getVotingStatus_impl, getSubmitPaperVote_impl};

#[macro_use]
mod common;
mod schema;
mod services;
mod context;

pub mod user_manager;
pub mod result_query;
pub mod submit_handler;
pub mod vote_data;

use crate::schema::{create_schema, Schema};

static KEY: OnceCell<ES256kKeyPair> = OnceCell::new();

fn read_a_file(filename: &str) -> std::io::Result<Vec<u8>> {
	let mut file = std::fs::File::open(filename)?;

	let mut data = Vec::new();
	file.read_to_end(&mut data)?;

	return Ok(data);
}

async fn graphiql_handler() -> Result<HttpResponse, Error> {
	gqli_handler("/graphql", None).await
}
async fn playground_handler() -> Result<HttpResponse, Error> {
	play_handler("/graphql", None).await
}
async fn graphql(
	req: actix_web::HttpRequest,
	payload: actix_web::web::Payload,
	schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
	//let vote_token = req.cookie("vote_token").map(|f| f.value().to_string());
	let ctx = Context {
		//vote_token: vote_token,
		additional_fingureprint: None,
		// TODO: additional fingerprint
		user_ip: req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string(),
		public_key: KEY.get().unwrap().clone()
	};
	graphql_handler(&schema, &ctx, req, payload).await
}


async fn user_token_status(body: actix_web::web::Json<user_manager::TokenStatusInputs>) -> Result<web::Json<user_manager::TokenStatusOutput>, Error> {
	let result = user_manager::user_token_status(body.user_token.clone(), body.vote_token.clone()).await;
	if result.is_ok() {
		let mut ret = user_manager::TokenStatusOutput { status: "valid".to_string(), voting_status: None, papers_json: None };
		let ctx = Context {
			//vote_token: vote_token,
			additional_fingureprint: None,
			// TODO: additional fingerprint
			user_ip: "".to_string(),
			public_key: KEY.get().unwrap().clone()
		};
		if let Some(vote_token) = &body.vote_token {
			let ret2 = getVotingStatus_impl(&ctx, vote_token.clone()).await;
			if let Ok(voting_status) = ret2 {
				ret.voting_status = Some(voting_status.clone());
				if voting_status.papers {
					ret.papers_json = Some(getSubmitPaperVote_impl(&ctx, vote_token.clone()).await.unwrap().papers_json);
				}
			}
		}
		Ok(web::Json(ret))
	} else {
		Ok(web::Json(user_manager::TokenStatusOutput { status: "invalid".to_string(), voting_status: None, papers_json: None }))
	}
}


async fn server_time() -> Result<String, Error> {
	let now = Utc::now().to_rfc3339();
	Ok(now.into())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	let key = ES256kKeyPair::from_pem(std::str::from_utf8(&read_a_file("../keys/key-priv.pem").unwrap()).unwrap()).unwrap();
	KEY.set(key).unwrap();

	// Start http server
	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(create_schema()))
			.wrap(
				Cors::default()
				.allow_any_origin()
				.allow_any_header()
				.allow_any_method()
			)
			// .wrap(middleware::Compress::default())
			// .wrap(middleware::Logger::default())
			.service(
				web::resource("/graphql")
					.route(web::post().to(graphql))
					.route(web::get().to(graphql)),
			)
			.service(web::resource("/playground").route(web::get().to(playground_handler)))
			.service(web::resource("/graphiql").route(web::get().to(graphiql_handler)))
			.service(web::resource("/user-token-status").route(web::post().to(user_token_status)))
			.service(web::resource("/server-time").route(web::get().to(server_time)))
	})
	.bind("0.0.0.0:80")?
	.run()
	.await
}