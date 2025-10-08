use crate::store::TicketId;
pub mod title;
pub mod description;
pub mod status;


pub use title::TicketTitle;
pub use description::TicketDescription;
pub use status::Status;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}


pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}