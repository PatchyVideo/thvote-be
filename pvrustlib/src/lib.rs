
use actix_web::{HttpResponse};
use actix_web::{http::StatusCode, ResponseError};
use juniper::{ScalarValue, FieldError, IntoFieldError, graphql_value, FieldResult};
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
            human_readable_message: None,
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

impl<S: ScalarValue> IntoFieldError<S> for ServiceError {
	fn into_field_error(self) -> FieldError<S> {
		match self {
			ServiceError::Error { resp } => {
                let service = resp.service;
                let url = resp.url;
                let error_kind = resp.error_kind;
                let error_message = resp.error_message;
                let human_readable_message = resp.human_readable_message;
                let upstream_response_string = resp.upstream_response_string;
                if let Some(upstream_response_json) = &resp.upstream_response_json {
                    println!("{:?}", upstream_response_json);
                }
				FieldError::new(
					"Error",
					graphql_value!({
                        "service": service,
                        "url": url,
                        "error_kind": error_kind,
                        "error_message": error_message,
                        "human_readable_message": human_readable_message,
                        "upstream_response_string": upstream_response_string
                    }),
				)
			}
		}
	}
}


pub async fn json_request<T: DeserializeOwned, J: serde::ser::Serialize>(service: &str, url: &str, obj: J) -> Result<T, ServiceError> {
    let client = reqwest::Client::new();
	let response = client.post(url)
		.json(&obj)
		.send()
		.await;
	let response = match response {
		Ok(r) => r,
		Err(e) => { return Err(ServiceError::new_network_error(service, url, Some(format!("{:?}", e)))); }
	};
	let status = response.status();
    if status.is_success() {
		let ret: T = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(service, url, Some(format!("{:?}", e)))); }
		};
		Ok(ret)
	} else {
		let err: ServiceErrorResponse = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(service, url, Some(format!("{:?}", e)))); }
		};
		Err(err.to_service_error())
	}
}

pub async fn json_request_gateway<T: DeserializeOwned, J: serde::ser::Serialize>(service: &str, url: &str, obj: J) -> FieldResult<T> {
    let client = reqwest::Client::new();
	let response = client.post(url)
		.json(&obj)
		.send()
		.await;
	let response = match response {
		Ok(r) => r,
		Err(e) => { return Err(ServiceError::new_network_error(service, url, Some(format!("{:?}", e))).into_field_error()); }
	};
	let status = response.status();
    if status.is_success() {
		let ret: T = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(service, url, Some(format!("{:?}", e))).into_field_error()); }
		};
		Ok(ret)
	} else {
		let err: ServiceErrorResponse = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(service, url, Some(format!("{:?}", e))).into_field_error()); }
		};
		Err(err.to_service_error().into_field_error())
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmptyJSON {
	
}
impl EmptyJSON {
	pub fn new() -> EmptyJSON {
		EmptyJSON {  }
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
