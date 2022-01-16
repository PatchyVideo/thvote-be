use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EmailRequest {
    pub code: String,
    pub email: String
}

