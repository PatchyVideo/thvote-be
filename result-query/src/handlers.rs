
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

pub async fn chars_reasons(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::ReasonsRequest>) -> Result<web::Json<models::ReasonsResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::chars_reasons(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}

pub async fn chars_trend(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::TrendRequest>) -> Result<web::Json<models::TrendResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::chars_trend(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.name.clone()).await?;
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

pub async fn musics_reasons(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::ReasonsRequest>) -> Result<web::Json<models::ReasonsResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::musics_reasons(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}

pub async fn musics_trend(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::TrendRequest>) -> Result<web::Json<models::TrendResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::musics_trend(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.name.clone()).await?;
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

pub async fn cps_reasons(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::ReasonsRequest>) -> Result<web::Json<models::ReasonsResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::cps_reasons(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}

pub async fn cps_trend(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::TrendRequest>) -> Result<web::Json<models::TrendResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::cps_trend(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.name.clone()).await?;
	Ok(web::Json(resp))
}

pub async fn global_stats(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::GlobalStatsRequest>) -> Result<web::Json<models::GlobalStats>, ServiceError> {
	let resp = query::global_stats(&ctx, bson::DateTime::from_chrono(body.vote_start), body.vote_year).await?;
	Ok(web::Json(resp))
}
