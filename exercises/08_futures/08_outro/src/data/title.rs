use std::fmt::{Display, Formatter};
use thiserror;

pub const MAX_TITLE_LEN: usize = 50;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketTitle(String);

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        validate(&value)?;
        Ok(TicketTitle(value))
    }
}


impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(TicketTitle(value))
    }
}


impl Display for TicketTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

fn validate(value: &String) -> Result<(), TicketTitleError> {
    if value.trim().is_empty() {
        Err(TicketTitleError::NoTitle)
    } else if value.len() > MAX_TITLE_LEN {
        Err(TicketTitleError::TitleTooLong)
    } else {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum TicketTitleError {
    #[error("Ticket title is empty!")]
    NoTitle,
    #[error("Ticket title is too long! It must be {MAX_TITLE_LEN} bytes!")]
    TitleTooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_its_possible_to_create_title_from_str(){
        assert_eq!(
            TicketTitle::try_from("John Doe - This is it!").unwrap(),
            TicketTitle("John Doe - This is it!".into()));
    }

    #[test]
    fn check_if_its_possible_to_create_title_from_string(){
        assert_eq!(
            TicketTitle::try_from("John Doe - This is it!".to_string()).unwrap(),
            TicketTitle("John Doe - This is it!".into()));
    }

    #[test]
    fn check_if_empty_title_errors(){
        assert_eq!(
            TicketTitle::try_from("").unwrap_err().to_string(),
            "Ticket title is empty!".to_string()
        )
    }

    #[test]
    fn check_if_long_title_errors(){
        let value: String = (0..=MAX_TITLE_LEN).map(|_| "a").collect();
        assert_eq!(
            TicketTitle::try_from(value).unwrap_err().to_string(),
            format!("Ticket title is too long! It must be {MAX_TITLE_LEN} bytes!")
        )
    }
    
}