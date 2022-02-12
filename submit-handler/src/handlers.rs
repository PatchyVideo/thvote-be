
use actix_web::{web, HttpRequest};
use pvrustlib::{EmptyJSON, ServiceError};

use crate::{models, common::{rate_limit, SERVICE_NAME}};

type SubmitServiceV1Wrapper = web::Data<crate::services::SubmitServiceV1>;

pub async fn submit_character_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::CharacterSubmitRest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let mut conn = service.redis_client.get_async_connection().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	rate_limit(&body.0.meta.vote_id, &mut conn).await?;
	let lockid = format!("lock-submit_character_v1-{}", body.0.meta.vote_id);
	let guard = service.lock.acquire_async(lockid.as_bytes(), 10 * 1000).await;
	let sanitized = service.validator.validate_character(body.0, &service.character_coll).await?;
	service.submit_charcater(sanitized).await?;
	Ok(web::Json(EmptyJSON::new()))
}

pub async fn submit_music_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::MusicSubmitRest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let mut conn = service.redis_client.get_async_connection().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	rate_limit(&body.0.meta.vote_id, &mut conn).await?;
	let lockid = format!("lock-submit_music_v1-{}", body.0.meta.vote_id);
	let guard = service.lock.acquire_async(lockid.as_bytes(), 10 * 1000).await;
	let sanitized = service.validator.validate_music(body.0, &service.music_coll).await?;
	service.submit_music(sanitized).await?;
	Ok(web::Json(EmptyJSON::new()))
}

pub async fn submit_cp_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::CPSubmitRest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let mut conn = service.redis_client.get_async_connection().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	rate_limit(&body.0.meta.vote_id, &mut conn).await?;
	let lockid = format!("lock-submit_cp_v1-{}", body.0.meta.vote_id);
	let guard = service.lock.acquire_async(lockid.as_bytes(), 10 * 1000).await;
	let sanitized = service.validator.validate_cp(body.0, &service.cp_coll).await?;
	service.submit_cp(sanitized).await?;
	Ok(web::Json(EmptyJSON::new()))
}

pub async fn submit_paper_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::PaperSubmitRest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let mut conn = service.redis_client.get_async_connection().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	rate_limit(&body.0.meta.vote_id, &mut conn).await?;
	let lockid = format!("lock-submit_paper_v1-{}", body.0.meta.vote_id);
	let guard = service.lock.acquire_async(lockid.as_bytes(), 10 * 1000).await;
	let sanitized = service.validator.validate_paper(body.0, &service.paper_coll).await?;
	service.submit_paper(sanitized).await?;
	Ok(web::Json(EmptyJSON::new()))
}

pub async fn submit_dojin_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::DojinSubmitRest>) -> Result<web::Json<EmptyJSON>, ServiceError> {
	let mut conn = service.redis_client.get_async_connection().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	rate_limit(&body.0.meta.vote_id, &mut conn).await?;
	let lockid = format!("lock-submit_character_v1-{}", body.0.meta.vote_id);
	let guard = service.lock.acquire_async(lockid.as_bytes(), 10 * 1000).await;
	let sanitized = service.validator.validate_dojin(body.0, &service.dojin_coll).await?;
	service.submit_dojin(sanitized).await?;
	Ok(web::Json(EmptyJSON::new()))
}

pub async fn get_submit_character_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::QuerySubmitRequest>) -> Result<web::Json<models::CharacterSubmitRest>, ServiceError> {
	let lockid = format!("lock-submit_character_v1-{}", body.0.vote_id);
	Ok(web::Json(service.get_submit_charcater(body.0.vote_id).await?))
}

pub async fn get_submit_music_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::QuerySubmitRequest>) -> Result<web::Json<models::MusicSubmitRest>, ServiceError> {
	let lockid = format!("lock-submit_music_v1-{}", body.0.vote_id);
	Ok(web::Json(service.get_submit_music(body.0.vote_id).await?))
}

pub async fn get_submit_cp_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::QuerySubmitRequest>) -> Result<web::Json<models::CPSubmitRest>, ServiceError> {
	let lockid = format!("lock-submit_cp_v1-{}", body.0.vote_id);
	Ok(web::Json(service.get_submit_cp(body.0.vote_id).await?))
}

pub async fn get_submit_paper_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::QuerySubmitRequest>) -> Result<web::Json<models::PaperSubmitRest>, ServiceError> {
	let lockid = format!("lock-submit_paper_v1-{}", body.0.vote_id);
	Ok(web::Json(service.get_submit_paper(body.0.vote_id).await?))
}

pub async fn get_submit_dojin_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::QuerySubmitRequest>) -> Result<web::Json<models::DojinSubmitRest>, ServiceError> {
	let lockid = format!("lock-submit_dojin_v1-{}", body.0.vote_id);
	Ok(web::Json(service.get_submit_dojin(body.0.vote_id).await?))
}

pub async fn get_voting_status_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::QuerySubmitRequest>) -> Result<web::Json<models::VotingStatus>, ServiceError> {
	Ok(web::Json(service.get_voting_status(body.0.vote_id).await?))
}
