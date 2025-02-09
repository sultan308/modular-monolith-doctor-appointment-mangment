use axum::body::Body;
use axum::http::{StatusCode};
use axum::Json;
use axum::response::{Response, IntoResponse};

use crate::errors::error_payload::ErrorPayload;

#[derive(Debug)]
pub enum ApplicationError {
    InvalidRequestPayload(String, Box<dyn std::error::Error + Send+ Sync>),
    InvalidOperation(String, Box<dyn std::error::Error + Send+ Sync>),
    RequestNotFound(String, Box<dyn std::error::Error + Send+ Sync>),
    InternalServerError(Box<dyn std::error::Error + Send+ Sync>),
}

impl ApplicationError {
    pub fn get_http_status_code(&self) -> StatusCode{
        match self {
            ApplicationError::InvalidRequestPayload(_, _) => StatusCode::BAD_REQUEST,
            ApplicationError::InvalidOperation(_, _) => StatusCode::BAD_REQUEST,
            ApplicationError::RequestNotFound(_, _) => StatusCode::NOT_FOUND,
            ApplicationError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn get_json_serializable_payload(&self) -> ErrorPayload{
        match self {
            ApplicationError::InvalidRequestPayload(msg,err)|
            ApplicationError::InvalidOperation(msg,err)|
            ApplicationError::RequestNotFound(msg,err) =>{
                ErrorPayload{message: msg.to_string()}
            },
            ApplicationError::InternalServerError(_) => {
                ErrorPayload{message: "internal server error.".to_string()}
            }

        }

    }

}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApplicationError::InvalidRequestPayload(_msg, err) => {
                write!(f,"ApplicationError::InvalidRequestPayload: {err}")
            },
            ApplicationError::InvalidOperation(_msg, err) => {
                write!(f,"ApplicationError::InvalidOperation: {err}")
            },
            ApplicationError::RequestNotFound(_msg, err) => {
                write!(f,"ApplicationError::RequestNotFound: {err}")
            },
            ApplicationError::InternalServerError( err) => {
                write!(f,"ApplicationError::InternalServerError: {err}")
            },
        }
    }
}

impl std::error::Error  for ApplicationError {}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response<Body> {
        let status_code = self.get_http_status_code();
        let response = self.get_json_serializable_payload();
        (status_code, Json(response)).into_response()

    }
}