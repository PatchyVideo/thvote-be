
#[cfg(debug_assertions)]
pub const MONGO_ADDRESS: &'static str = "mongodb://192.168.0.54:27017";

#[cfg(not(debug_assertions))]
pub const MONGO_ADDRESS: &'static str = "mongodb://mongo:27017";

#[cfg(debug_assertions)]
pub const REDIS_ADDRESS: &'static str = "redis://192.168.0.54:6379";

#[cfg(not(debug_assertions))]
pub const REDIS_ADDRESS: &'static str = "redis://redis:6379";

#[cfg(debug_assertions)]
pub const SERVICE_SMS_ADDRESS: &'static str = "http://127.0.0.1:5010";

#[cfg(not(debug_assertions))]
pub const SERVICE_SMS_ADDRESS: &'static str = "http://sms-service";

#[cfg(debug_assertions)]
pub const SERVICE_EMAIL_ADDRESS: &'static str = "http://127.0.0.1:5011";

#[cfg(not(debug_assertions))]
pub const SERVICE_EMAIL_ADDRESS: &'static str = "http://email-service";
