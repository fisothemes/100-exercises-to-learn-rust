use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::data::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("{0}: {1}")]
    HttpStatusCode(StatusCode, String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ticket title error: {0}")]
    Title(#[from] title::TicketTitleError),
    #[error("Ticket description error: {0}")]
    Description(#[from] description::TicketDescriptionError),
    #[error("Ticket status error: {0}")]
    Status(#[from] status::StatusError)
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        
        let (status, message) = match self {
            Self::JsonParse(message) => (StatusCode::BAD_REQUEST, message.to_string()),
            Self::HttpStatusCode(status, message) => (status, message),
            Self::Title(message) => (StatusCode::BAD_REQUEST, message.to_string()),
            Self::Description(message) => (StatusCode::BAD_REQUEST, message.to_string()),
            Self::Status(message) => (StatusCode::BAD_REQUEST, message.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
        };

        let body = Json(json!({ "error" : message }));

        (status, body).into_response()
    }
}