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
    #[error("Ticket title error: {0}")]
    Title(#[from] title::TicketTitleError),
    #[error("Ticket description error: {0}")]
    Description(#[from] description::TicketDescriptionError),
    #[error("Ticket status error: {0}")]
    Status(#[from] status::StatusError)
}