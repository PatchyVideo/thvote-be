
pub mod models;
pub mod context;
pub mod jwt;
pub mod comm;
pub mod common;
pub mod handlers;

pub mod sms_service;
pub mod email_service;

pub mod legacy_login;
pub mod new_login;
pub mod thbwiki_login;
pub mod qq_binding;

pub mod account_management;

use std::{cell::Cell, sync::Arc};

use actix_web::{App, HttpRequest, HttpServer, Responder, web::{self, Data}};
use context::AppContext;
use jwt::load_keys;
use models::ActivityLogEntry;
use mongodb::{Client, options::ClientOptions};
use serde::{Deserialize, Serialize};

use redis::AsyncCommands;

pub async fn log(ctx: &AppContext, log: ActivityLogEntry) {
    ctx.logs_coll.insert_one(log, None).await;
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config_vote_date {
    pub vote_year: u32,
    pub vote_start: String,
    pub vote_end: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub vote_date: Config_vote_date,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config: Config = toml::from_str(&std::fs::read_to_string("../keys/config.toml").unwrap()).expect("Config must be a valid toml file");
    let vote_start = chrono::DateTime::parse_from_rfc3339(&config.vote_date.vote_start).unwrap().with_timezone(&chrono::Utc);
    let vote_end = chrono::DateTime::parse_from_rfc3339(&config.vote_date.vote_end).unwrap().with_timezone(&chrono::Utc);

    let client_options = ClientOptions::parse(comm::MONGO_ADDRESS).await.expect("Failed to parse MongoDB parameters");
	let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");

	let db = client.database("thvote_users");

    let redis_client = redis::Client::open(comm::REDIS_ADDRESS).unwrap();

    let ctx = context::AppContext {
        vote_year: config.vote_date.vote_year,
        vote_start: vote_start,
        vote_end: vote_end,
        db: db.clone(),
        voters_coll: db.collection("voters"),
        logs_coll: db.collection("voter_logs"),
        redis_client: redis_client,
        key_pair: load_keys().await.unwrap(),
    };
    HttpServer::new(move || {
        App::new().app_data(Data::new(ctx.clone()))
            .route("/v1/login-email-password", web::post().to(handlers::login_email_password))
            .route("/v1/login-email", web::post().to(handlers::login_email))
            .route("/v1/login-phone", web::post().to(handlers::login_phone))
            .route("/v1/update-email", web::post().to(handlers::update_email))
            .route("/v1/update-phone", web::post().to(handlers::update_phone))
            .route("/v1/update-nickname", web::post().to(handlers::update_nickname))
            .route("/v1/update-password", web::post().to(handlers::update_password))
            .route("/v1/send-sms-code", web::post().to(handlers::send_phone_verify_code))
            .route("/v1/send-email-code", web::post().to(handlers::send_email_verify_code))
            .route("/v1/user-token-status", web::post().to(handlers::user_token_status))
            .route("/v1/remove-voter", web::post().to(handlers::remove_voter))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
