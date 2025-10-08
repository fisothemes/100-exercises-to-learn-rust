use std::fmt::{Display, Formatter, Result};
use thiserror;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    #[default]
    ToDo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Self::ToDo => "To-do",
            Self::InProgress => "In progress",
            Self::Done => "Done"
        })
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "todo" | "to-do" => Ok(Self::ToDo),
            "in progress" | "in-progress" | "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(StatusError::ParseError(value.into()))
        }
    }
}
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum StatusError{
    #[error("Failed to parse \"{0}\" into Status type.")]
    ParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_status_string_representation(){
        assert_eq!(Status::ToDo.to_string(), "To-do".to_string());
        assert_eq!(Status::InProgress.to_string(), "In progress".to_string());
        assert_eq!(Status::Done.to_string(), "Done".to_string());
    }

    #[test]
    fn check_if_its_possible_to_create_status_from_string() {
        assert_eq!(Status::try_from("To-Do"), Ok(Status::ToDo));
        assert_eq!(Status::try_from("In Progress"), Ok(Status::InProgress));
        assert_eq!(Status::try_from("Done"), Ok(Status::Done));
        assert_eq!(
            Status::try_from("Invalid").unwrap_err().to_string(),
            "Failed to parse \"Invalid\" into Status type.".to_string()
        );
    }
}