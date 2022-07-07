
use std::str::FromStr;

use actix_web::{App, HttpMessage, HttpRequest, HttpServer, Responder, web};
use bson::oid::ObjectId;


use crate::{models::{self}, context::AppContext, service_error::ServiceError, query, common::SERVICE_NAME};


pub async fn chars_rank(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RankingQueryRequest>) -> Result<web::Json<models::RankingQueryResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::chars_ranking(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year).await?;
	Ok(web::Json(resp))
}

pub async fn musics_rank(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RankingQueryRequest>) -> Result<web::Json<models::RankingQueryResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::musics_ranking(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year).await?;
	Ok(web::Json(resp))
}

pub async fn cps_rank(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::RankingQueryRequest>) -> Result<web::Json<models::CPRankingQueryResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::cps_ranking(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year).await?;
	Ok(web::Json(resp))
}
