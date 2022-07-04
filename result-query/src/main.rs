

extern crate pest;
#[macro_use]
extern crate pest_derive;

use actix_web::{App, HttpRequest, HttpServer, Responder, web::{self, Data}};

mod models;
mod handlers;
mod parser;
mod comm;
mod context;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let client_options = ClientOptions::parse(comm::MONGO_ADDRESS).await.expect("Failed to parse MongoDB parameters");
	let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");

	let db = client.database("submits_v1_final");

    let ctx = context::AppContext {
        db: db,
        votes_coll: db.collection("votes"),
    };
    HttpServer::new(move || {
        App::new().app_data(Data::new(ctx.clone()))
            .route("/v1/chars-rank", web::post().to(handlers::chars_rank))
            .route("/v1/musics-rank", web::post().to(handlers::musics_rank))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
