
use actix_web::{HttpResponse};
use actix_web::{http::StatusCode, ResponseError};

use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ServiceError {
	#[error("Error")]
	Error { resp: ServiceErrorResponse },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceErrorResponse {
    pub service: String,
    pub url: Option<String>,
    pub error_kind: String,
    pub error_message: Option<String>,
    pub human_readable_message: Option<String>,
    pub upstream_response_json: Option<serde_json::Value>,
    pub upstream_response_string: Option<String>
}

impl ServiceErrorResponse {
    pub fn to_service_error(&self) -> ServiceError {
        ServiceError::Error { resp: self.clone() }
    }
}

impl ServiceError {
	pub fn name(&self) -> String {
		match self {
			Self::Error { resp: _ } => "Error".to_string(),
		}
	}
	pub fn from_resp(resp: ServiceErrorResponse) -> Self {
        Self::Error { resp: resp }
    }
    pub fn from_dyn_error(service: &str, err: Box<dyn std::error::Error>) -> Self {
        if let Some(service_error) = err.downcast_ref::<ServiceError>() {
            service_error.clone()
        } else {
            ServiceError::new(service, format!("{:?}", err))
        }
    }
    pub fn new_error_kind(service: &str, error_kind: &str) -> Self {
        let resp = ServiceErrorResponse {
            service: service.to_string(),
            url: None,
            error_kind: error_kind.to_string(),
            error_message: None,
            human_readable_message: None,
            upstream_response_json: None,
            upstream_response_string: None
        };
        ServiceError::Error { resp: resp }
    }
    pub fn new_network_error(service: &str, url: &str, errmsg: Option<String>) -> Self {
        let resp = ServiceErrorResponse {
            service: service.to_string(),
            url: Some(url.to_string()),
            error_kind: "NETWORK_ERROR".to_string(),
            error_message: errmsg.clone(),
            human_readable_message: None,
            upstream_response_json: None,
            upstream_response_string: None
        };
        ServiceError::Error { resp: resp }
    }
    pub fn new_json_error(service: &str, url: &str, errmsg: Option<String>) -> Self {
        let resp = ServiceErrorResponse {
            service: service.to_string(),
            url: Some(url.to_string()),
            error_kind: "JSON_DECODE_ERROR".to_string(),
            error_message: errmsg.clone(),
            human_readable_message: None,
            upstream_response_json: None,
            upstream_response_string: None
        };
        ServiceError::Error { resp: resp }
    }
    pub fn new_jwt_error(service: &str, errmsg: Option<String>) -> Self {
        let resp = ServiceErrorResponse {
            service: service.to_string(),
            url: None,
            error_kind: "INVALID_TOKEN".to_string(),
            error_message: errmsg.clone(),
            human_readable_message: Some("无效凭证或不在投票日期".into()),
            upstream_response_json: None,
            upstream_response_string: None
        };
        ServiceError::Error { resp: resp }
    }
    pub fn new(service: &str, errmsg: String) -> Self {
        let resp = ServiceErrorResponse {
            service: service.to_string(),
            url: None,
            error_kind: "INTERNAL_SERVER_ERROR".to_string(),
            error_message: Some(errmsg),
            human_readable_message: None,
            upstream_response_json: None,
            upstream_response_string: None
        };
        ServiceError::Error { resp: resp }
    }
    pub fn new_human_readable(service: &str, error_kind: &str, human_readable_message: String) -> Self {
        let resp = ServiceErrorResponse {
            service: service.to_string(),
            url: None,
            error_kind: error_kind.to_string(),
            error_message: None,
            human_readable_message: Some(human_readable_message),
            upstream_response_json: None,
            upstream_response_string: None
        };
        ServiceError::Error { resp: resp }
    }
    pub fn new_not_found(service: &str, resource_name: Option<String>) -> Self {
        let resp = ServiceErrorResponse {
            service: service.to_string(),
            url: None,
            error_kind: "NOT_FOUND".to_string(),
            error_message: resource_name,
            human_readable_message: None,
            upstream_response_json: None,
            upstream_response_string: None
        };
        ServiceError::Error { resp: resp }
    }
}

impl ResponseError for ServiceError {
	fn status_code(&self) -> StatusCode {
		match *self {
			Self::Error { resp: _ } => StatusCode::BAD_REQUEST
		}
	}

	fn error_response(&self) -> HttpResponse {
		let status_code = self.status_code();
        let resp = match self {
            ServiceError::Error { resp } => resp,
        };
		HttpResponse::build(status_code).json(resp)
	}
}
