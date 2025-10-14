use serde::{Deserialize, Serialize};
pub mod title;
pub mod description;
pub mod status;


pub use title::TicketTitle;
pub use description::TicketDescription;
pub use status::Status;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketPatch {
    pub id: TicketId,
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_json_serde_for_ticket_id() {
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
        struct SimpleTicket{ id: TicketId }

        let t = SimpleTicket{ id: TicketId(12) };

        let ser = serde_json::to_string(&t).unwrap();
        assert_eq!(r#"{"id":12}"#, ser, "Serialization failed for {t:?}");

        let de: SimpleTicket = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, t, "Deserialization failed for {t:?}");
    }

    #[test]
    fn check_json_serde_for_ticket_draft() {
        let t = TicketDraft {
            title: TicketTitle::try_from("Hello There!").unwrap(),
            description: TicketDescription::try_from("A Star Wars Story.").unwrap()
        };

        let ser = serde_json::to_string(&t).unwrap();
        assert_eq!(
            r#"{"title":"Hello There!","description":"A Star Wars Story."}"#,
            ser,
            "Serialization failed for {t:?}"
        );

        let de: TicketDraft = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, t, "Deserialization failed for {t:?}");
    }

    #[test]
    fn check_json_serde_for_ticket() {
        let t = Ticket {
            id: TicketId(33),
            title: TicketTitle::try_from("Jimmy").unwrap(),
            description: TicketDescription::try_from("A Neutron Story.").unwrap(),
            status: Status::InProgress
        };

        let ser = serde_json::to_string(&t).unwrap();
        assert_eq!(
            r#"{"id":33,"title":"Jimmy","description":"A Neutron Story.","status":"In progress"}"#,
            ser,
            "Serialization failed for {t:?}"
        );

        let de: Ticket = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, t, "Deserialization failed for {t:?}");
    }

    #[test]
    fn check_json_serde_for_ticket_patch(){
        let t = TicketPatch {
            id: TicketId(33),
            title: None,
            description: None,
            status: Some(Status::Done)
        };

        let ser = serde_json::to_string(&t).unwrap();
        assert_eq!(
            r#"{"id":33,"title":null,"description":null,"status":"Done"}"#,
            ser,
            "Serialization failed for {t:?}"
        );

        let de: TicketPatch = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, t, "Deserialization failed for {t:?}");
    }
}