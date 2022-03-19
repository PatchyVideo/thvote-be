
#[cfg(debug_assertions)]
pub const USER_MANAGER: &'static str = "127.0.0.1:1100";
#[cfg(debug_assertions)]
pub const SUBMIT_HANDLER: &'static str = "127.0.0.1:1101";
#[cfg(debug_assertions)]
pub const RESULT_QUERY: &'static str = "127.0.0.1:1102";
#[cfg(debug_assertions)]
pub const SUBMIT_VALIDATOR: &'static str = "127.0.0.1:1103";

#[cfg(not(debug_assertions))]
pub const USER_MANAGER: &'static str = "user-manager";
#[cfg(not(debug_assertions))]
pub const SUBMIT_HANDLER: &'static str = "submit-handler";
#[cfg(not(debug_assertions))]
pub const RESULT_QUERY: &'static str = "result-query";

