
#[cfg(debug_assertions)]
pub const REDIS_ADDRESS: &'static str = "redis://192.168.0.54:6379";

#[cfg(not(debug_assertions))]
pub const REDIS_ADDRESS: &'static str = "redis://redis:6379";
