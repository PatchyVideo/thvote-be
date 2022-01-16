
use chrono::Utc;
use pvrustlib::ServiceError;
use redis::AsyncCommands;

pub static SERVICE_NAME: &'static str = "submit-handler";

#[cfg(debug_assertions)]
pub const MONGODB_URL: &str = "mongodb://localhost:27017/";

#[cfg(not(debug_assertions))]
pub const MONGODB_URL: &str = "mongodb://mongo:27017/";


pub const RATE_LIMIT_WINDOW_SIZE_IN_SECONDS: i64 = 60;
pub const RATE_LIMIT_MAX_REQUETS: i64 = 30;

/// Rate limiting using token bucket
pub async fn rate_limit(uid: &impl std::fmt::Display, conn: &mut redis::aio::Connection) -> Result<(), ServiceError> {
	let cur_time = Utc::now().timestamp_millis();
	let id = format!("rate-limit-{}-last-reset", uid);
	let id_ctr = format!("rate-limit-{}-tokens", uid);
	let last_time: Option<i64> = conn.get(&id).await.map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, Box::new(e)))?;
	let (last_time, tokens_remaining) = if let Some(last_time) = last_time {
		let remain: Option<i64> = conn.get(&id_ctr).await.map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, Box::new(e)))?;
		(last_time, remain.unwrap())
	} else {
		conn.set(id.clone(), cur_time).await.map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, Box::new(e)))?;
		conn.set(id_ctr.clone(), RATE_LIMIT_MAX_REQUETS).await.map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, Box::new(e)))?;
		(cur_time, RATE_LIMIT_MAX_REQUETS)
	};
	if cur_time - last_time > RATE_LIMIT_WINDOW_SIZE_IN_SECONDS * 1000 {
		// reset bucket
		conn.set(id.clone(), cur_time).await.map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, Box::new(e)))?;
		conn.set(id_ctr.clone(), RATE_LIMIT_MAX_REQUETS).await.map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, Box::new(e)))?;
	} else {
		if tokens_remaining <= 0 {
			return Err(ServiceError::new_error_kind(SERVICE_NAME, "REQUEST_TOO_FREQUENT").into());
		}
	}
	conn.decr(id_ctr.clone(), 1).await.map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, Box::new(e)))?;
	Ok(())
}

