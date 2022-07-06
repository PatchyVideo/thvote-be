
use std::str::FromStr;

use actix_web::{App, HttpMessage, HttpRequest, HttpServer, Responder, web};
use bson::oid::ObjectId;


use crate::{models::{self}, context::AppContext, service_error::ServiceError, query};


pub async fn chars_rank(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RankingQueryRequest>) -> Result<web::Json<models::RankingQueryResponse>, ServiceError> {
	let resp = query::chars_ranking(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year).await?;
	Ok(web::Json(resp))
}

pub async fn musics_rank(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RankingQueryRequest>) -> Result<web::Json<models::RankingQueryResponse>, ServiceError> {
	todo!();
}
