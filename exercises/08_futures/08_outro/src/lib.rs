// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

pub mod data;
pub mod store;
pub mod client;
pub mod error;
pub mod server;

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::time::Duration;
    use crate::client::Client;
    use crate::data::{Status, Ticket, TicketDraft, TicketPatch};
    use crate::server::Server;
    use super::*;

    #[tokio::test]
    async fn it_works() -> error::Result<()> {
        let server = Server::serve("127.0.0.1:20202").await?;
        let addr = server.local_addr()?;

        tokio::spawn(async { server.await });

        tokio::time::sleep(Duration::from_secs(2)).await;

        launch_client(addr).await
    }

    // Test helper function.
    async fn launch_client(addr: SocketAddr) -> error::Result<()> {
        let c = Client::with_addr(addr.to_string())?;

        let test_ticket = Ticket::with(0.into(), "Cats", "The movie!", Status::ToDo)?;

        let id = c.create(&TicketDraft::with("Cats", "The movie!")?).await?;

        assert_eq!(test_ticket.id, id, "Ticket creation test, ticket id received is: {id}");

        let ticket = c.retrieve(id).await?;

        assert_eq!(test_ticket, ticket, "Ticket retrieval test, ticket received is: {ticket:#?}");

        let ticket_patch = TicketPatch{
            id,
            title: None,
            description: None,
            status: Some(Status::InProgress),
        };

        let ticket = c.patch(ticket_patch).await?;

        assert_eq!(Status::InProgress, ticket.status, "Ticket patch test, ticket received is: {ticket:#?}");

        let ticket = c.retrieve(id).await?;

        assert_eq!(
            Status::InProgress, ticket.status,
            "2nd Ticket retrieval test to ensure ticket changed, ticket received is: {ticket:#?}"
        );

        Ok(())
    }
}