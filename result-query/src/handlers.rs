
use std::str::FromStr;

use actix_web::{App, HttpMessage, HttpRequest, HttpServer, Responder, web};
use bson::oid::ObjectId;
use jwt_simple::prelude::{Claims, ECDSAP256kPublicKeyLike};
use pvrustlib::{ServiceError, EmptyJSON};

use crate::models::{self};


pub async fn chars_rank(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RankingQueryRequest>) -> Result<web::Json<models::RankingQueryResponse>, ServiceError> {
	let resp = models::RankingQueryResponse {
		entries: vec![]
	};
	Ok(web::Json(resp))
}

pub async fn musics_rank(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RankingQueryRequest>) -> Result<web::Json<models::RankingQueryResponse>, ServiceError> {
	let resp = models::RankingQueryResponse {
		entries: vec![]
	};
	Ok(web::Json(resp))
}
