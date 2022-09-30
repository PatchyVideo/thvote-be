

extern crate pest;
#[macro_use]
extern crate pest_derive;

use actix_web::{App, HttpRequest, HttpServer, Responder, web::{self, Data}};
use mongodb::{options::ClientOptions, Client};

mod service_error;
mod models;
mod handlers;
mod parser;
mod comm;
mod context;
mod query;
mod common;

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let client_options = ClientOptions::parse(comm::MONGO_ADDRESS).await.expect("Failed to parse MongoDB parameters");
	let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");

	let db = client.database("submits_v1_final");

    let redlock = redlock::RedLock::new(vec![comm::REDIS_ADDRESS]);

    let ctx = context::AppContext {
        db: db.clone(),
        votes_coll: db.collection("votes"),
        lock: redlock,
        chars_entry_cache_coll: db.collection("cache_chars_entry"),
        chars_global_cache_coll: db.collection("cache_chars_global"),
        musics_entry_cache_coll: db.collection("cache_musics_entry"),
        musics_global_cache_coll: db.collection("cache_musics_global"),
        cps_entry_cache_coll: db.collection("cache_cps_entry"),
        cps_global_cache_coll: db.collection("cache_cps_global"),
        all_chars: db.collection("chars"),
        all_musics: db.collection("musics"),
        global_stats: db.collection("global_stats"),
        completion_rates: db.collection("completion_rates"),
        paper_result: db.collection("paper_result"),
        covote_chars: db.collection("covote_chars"),
        covote_musics: db.collection("covote_musics"),
    };
    HttpServer::new(move || {
        App::new().app_data(Data::new(ctx.clone()))
            .route("/v1/chars-rank/", web::post().to(handlers::chars_rank))
            .route("/v1/musics-rank/", web::post().to(handlers::musics_rank))
            .route("/v1/cps-rank/", web::post().to(handlers::cps_rank))
            .route("/v1/chars-reasons/", web::post().to(handlers::chars_reasons))
            .route("/v1/musics-reasons/", web::post().to(handlers::musics_reasons))
            .route("/v1/cps-reasons/", web::post().to(handlers::cps_reasons))
            .route("/v1/chars-trend/", web::post().to(handlers::chars_trend))
            .route("/v1/musics-trend/", web::post().to(handlers::musics_trend))
            .route("/v1/cps-trend/", web::post().to(handlers::cps_trend))
            .route("/v1/global-stats/", web::post().to(handlers::global_stats))
            .route("/v1/completion-rates/", web::post().to(handlers::completion_rates))
            .route("/v1/papers/", web::post().to(handlers::paper_query))
            .route("/v1/papers-trend/", web::post().to(handlers::paper_trend))
            .route("/v1/chars-covote/", web::post().to(handlers::chars_covote))
            .route("/v1/musics-covote/", web::post().to(handlers::musics_covote))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
