
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
	if body.rank <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
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
	if body.rank <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
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
	if body.rank <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
	let resp = query::cps_reasons(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}

pub async fn cps_trend(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::TrendRequestRank>) -> Result<web::Json<models::TrendResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	if body.rank <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
	let resp = query::cps_trend(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}

pub async fn global_stats(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::GlobalStatsRequest>) -> Result<web::Json<models::GlobalStats>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::global_stats(&ctx, bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.query.clone()).await?;
	Ok(web::Json(resp))
}

pub async fn completion_rates(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::CompletionRateRequest>) -> Result<web::Json<models::CompletionRate>, ServiceError> {
	let resp = query::completion_rates(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year).await?;
	Ok(web::Json(resp))
}


pub async fn paper_query(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::QueryQuestionnaireRequest>) -> Result<web::Json<models::QueryQuestionnaireResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	if body.questions_of_interest.len() == 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "NO_QUESTIONNAIRE_REQUESTED", "没有问题查询".into()));
	}
	let resp = query::paper_result(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.questions_of_interest.clone()).await?;
	Ok(web::Json(resp))
}

pub async fn paper_trend(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::TrendRequest>) -> Result<web::Json<models::TrendResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	let resp = query::papers_trend(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.name.clone()).await?;
	Ok(web::Json(resp))
}

pub async fn chars_covote(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::CovoteRequest>) -> Result<web::Json<models::CovoteResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	if body.first_k <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
	let resp = query::chars_covote(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.first_k).await?;
	Ok(web::Json(resp))
}

pub async fn musics_covote(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::CovoteRequest>) -> Result<web::Json<models::CovoteResponse>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	if body.first_k <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
	let resp = query::musics_covote(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.first_k).await?;
	Ok(web::Json(resp))
}

pub async fn chars_single(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::SingleRankQuery>) -> Result<web::Json<models::RankingEntry>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	if body.rank <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
	let resp = query::chars_single(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}

pub async fn musics_single(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::SingleRankQuery>) -> Result<web::Json<models::RankingEntry>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	if body.rank <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
	let resp = query::musics_single(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}

pub async fn cps_single(ctx: web::Data<AppContext>, request: HttpRequest, body: actix_web::web::Json<models::SingleRankQuery>) -> Result<web::Json<models::CPRankingEntry>, ServiceError> {
	if body.query.as_ref().map(|f| f.len()).unwrap_or_default() > 1000 {
		// query too long
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "QUERY_TOO_LONG", "查询过长".into()));
	};
	if body.rank <= 0 {
		return Err(ServiceError::new_human_readable(SERVICE_NAME, "INVALID_K", "无效的k参数".into()));
	}
	let resp = query::cps_single(&ctx, body.query.clone(), bson::DateTime::from_chrono(body.vote_start), body.vote_year, body.rank).await?;
	Ok(web::Json(resp))
}
