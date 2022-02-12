
use actix_web::{web::{self, Data}, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use tokio::runtime::Handle;

mod comm;
mod common;
mod models;
mod handlers;
mod services;
mod validator;
mod paper_validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "actix_web=debug");
    let client_options = ClientOptions::parse(common::MONGODB_URL).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("submits_v1");

    let redlock = redlock::RedLock::new(vec![comm::REDIS_ADDRESS]);
    let redis_client = redis::Client::open(comm::REDIS_ADDRESS).unwrap();
    let submit_service_v1 = services::SubmitServiceV1::new(db.clone(), redis_client, redlock).await;

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(submit_service_v1.clone()))
            .route("/v1/character/", web::post().to(handlers::submit_character_v1))
            .route("/v1/music/", web::post().to(handlers::submit_music_v1))
            .route("/v1/cp/", web::post().to(handlers::submit_cp_v1))
            .route("/v1/paper/", web::post().to(handlers::submit_paper_v1))
            .route("/v1/dojin/", web::post().to(handlers::submit_dojin_v1))
            .route("/v1/get-character/", web::post().to(handlers::get_submit_character_v1))
            .route("/v1/get-music/", web::post().to(handlers::get_submit_music_v1))
            .route("/v1/get-cp/", web::post().to(handlers::get_submit_cp_v1))
            .route("/v1/get-paper/", web::post().to(handlers::get_submit_paper_v1))
            .route("/v1/get-dojin/", web::post().to(handlers::get_submit_dojin_v1))
            .route("/v1/voting-status/", web::post().to(handlers::get_voting_status_v1))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
