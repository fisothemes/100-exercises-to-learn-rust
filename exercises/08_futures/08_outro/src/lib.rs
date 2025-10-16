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
    use super::{
        server::Server,
        error::Result,
    };

    #[tokio::test]
    async fn it_works() -> Result<()> {
        let server = Server::new().serve("127.0.0.1:20202").await?;

        println!("Listening on http://{}", server.local_addr()?);

        assert!(false);

        Ok(server.await?)
    }
}