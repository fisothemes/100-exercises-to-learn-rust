use std::fmt::{Display, Formatter};
use thiserror;
use serde::{Serialize, Deserialize};

pub const MAX_DESCRIPTION_LEN: usize = 500;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketDescription(String);


impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        validate(&value)?;
        Ok(TicketDescription(value))
    }
}


impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(TicketDescription(value))
    }
}


impl Display for TicketDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
fn validate(value: &str) -> Result<(), TicketDescriptionError> {
    if value.trim().is_empty() {
        Err(TicketDescriptionError::NoDescription)
    } else if value.len() > MAX_DESCRIPTION_LEN {
        Err(TicketDescriptionError::DescriptionTooLong)
    } else {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum TicketDescriptionError {
    #[error("Ticket description is empty!")]
    NoDescription,
    #[error("Ticket description is too long! It must be {MAX_DESCRIPTION_LEN} bytes!")]
    DescriptionTooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_its_possible_to_create_description_from_str(){
        assert_eq!(
            TicketDescription::try_from("A cat concert").unwrap(),
            TicketDescription("A cat concert".into()));
    }

    #[test]
    fn check_if_its_possible_to_create_description_from_string(){
        assert_eq!(
            TicketDescription::try_from("A cat concert".to_string()).unwrap(),
            TicketDescription("A cat concert".into()));
    }

    #[test]
    fn check_if_empty_description_errors(){
        assert_eq!(
            TicketDescription::try_from("").unwrap_err().to_string(),
            "Ticket description is empty!".to_string()
        )
    }

    #[test]
    fn check_if_long_description_errors(){
        let value: String = (0..=MAX_DESCRIPTION_LEN).map(|_| "a").collect();
        assert_eq!(
            TicketDescription::try_from(value).unwrap_err().to_string(),
            format!("Ticket description is too long! It must be {MAX_DESCRIPTION_LEN} bytes!")
        )
    }

    #[test]
    fn check_json_serde_for_ticket_description() {
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
        struct SimpleTicket{
            desc: TicketDescription,
        }
        let t = SimpleTicket{
            desc: TicketDescription::try_from("A sung-through musical...").unwrap()
        };

        let ser = serde_json::to_string(&t).unwrap();
        assert_eq!(
            r#"{"desc":"A sung-through musical..."}"#, ser, "Serialization failed for {t:?}"
        );

        let de: SimpleTicket = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, t, "Deserialization failed for {t:?}");
    }
}