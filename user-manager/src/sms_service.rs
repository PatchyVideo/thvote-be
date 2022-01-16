
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SMSRequest {
    pub code: String,
    pub mobile: String
}
