use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tokio::sync::{RwLock};
use serde::{Serialize, Deserialize};

use crate::data::{Status, TicketId, Ticket, TicketDraft};



impl Display for TicketId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Arc<RwLock<Ticket>>> {
        self.tickets.get_mut(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Status, TicketDescription, TicketDraft, TicketTitle};
    use tokio::task;
    use serde::{Serialize, Deserialize};
    use serde_json;

    fn create_draft(title: &str, description: &str) -> TicketDraft {
        TicketDraft {
            title: TicketTitle::try_from(title).unwrap(),
            description: TicketDescription::try_from(description).unwrap()
        }
    }

    #[test]
    fn check_if_add_ticket_returns_incrementing_ids_starting_from_zero() {
        let mut store = TicketStore::new();

        let id0 = store.add_ticket(create_draft("First", "First ticket"));
        let id1 = store.add_ticket(create_draft("Second", "Second ticket"));
        let id2 = store.add_ticket(create_draft("Third", "Third ticket"));

        assert_eq!(id0, TicketId(0));
        assert_eq!(id1, TicketId(1));
        assert_eq!(id2, TicketId(2));
    }

    #[tokio::test]
    async fn check_if_get_returns_inserted_ticket_with_default_status() {
        let mut store = TicketStore::new();

        let id = store.add_ticket(create_draft("The thing", "A very scary movie..."));
        let got = store.get(id).unwrap();

        let read_guard = got.read().await;

        assert_eq!(read_guard.id, id);
        assert_eq!(read_guard.status, Status::ToDo);
        assert_eq!(read_guard.description.to_string(), "A very scary movie...");
    }

    #[test]
    fn check_if_get_unknown_id_returns_none() {
        let store = TicketStore::new();
        assert!(store.get(TicketId(42)).is_none());
    }

    #[tokio::test]
    async fn check_if_get_mut_provides_mutable_arc_and_allows_status_update() {
        let mut store = TicketStore::new();

        let id = store.add_ticket(create_draft("The Science", "A documentary about science."));
        let ticket = store.get_mut(id).unwrap();

        {
            let mut write_guard = ticket.write().await;
            write_guard.status = Status::InProgress;
        }

        let another_ticket = store.get(id).unwrap();
        let read_guard = another_ticket.read().await;
        assert_eq!(read_guard.status, Status::InProgress);
    }


    #[tokio::test]
    async fn check_if_multiple_tasks_can_read_concurrently(){
        let mut store = TicketStore::new();

        let id = store.add_ticket(create_draft("The Parallel", "A check on RwLock."));
        let a = store.get(id).unwrap();
        let b = store.get(id).unwrap();

        let (r1, r2) = tokio::join!(
            task::spawn(async move {
                let g = a.read().await;
                (g.id, g.status)
            }),
            task::spawn(async move {
                let g = b.read().await;
                (g.id, g.status)
            }),
        );

        let (id1, status1) = r1.unwrap();
        let (id2, status2) = r2.unwrap();

        assert_eq!(id1, id);
        assert_eq!(id2, id);
        assert_eq!(status1, Status::ToDo);
        assert_eq!(status2, Status::ToDo);
    }

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
}

