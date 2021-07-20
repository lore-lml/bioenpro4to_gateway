use std::error::Error;
use derive_more::{Display};
use actix_web::{ResponseError as AWResponseError, HttpResponse};

#[allow(dead_code)]
#[derive(Display, Debug)]
pub enum ResponseError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Internal(String),
}

impl Error for ResponseError {}

impl AWResponseError for ResponseError{
    fn error_response(&self) -> HttpResponse {
        match self {
            ResponseError::NotFound(err) => HttpResponse::NotFound().body(err),
            ResponseError::BadRequest(err) => HttpResponse::BadRequest().body(err),
            ResponseError::Unauthorized(err) => HttpResponse::Unauthorized().body(err),
            ResponseError::Internal(err) => HttpResponse::InternalServerError().body(err),
        }
    }
}
