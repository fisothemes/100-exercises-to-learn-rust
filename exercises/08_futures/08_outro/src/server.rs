use tokio::net::{ToSocketAddrs, TcpListener};
use serde_json::Value;
use axum::{
    Router,
    Json,
    routing::{get, post, patch},
    serve::Serve,
    extract::{State},
};
use crate::{
    error::Result,
    store::TicketStore,
    data::{TicketId, Ticket},
};
#[derive(Debug)]
pub struct Server{
    router: Router
}

impl Server {
    pub fn new() -> Self {

        let base_path = "/tickets";

        let router = Router::new()
            .route(base_path, get(Self::list_all))
            .route(base_path, post(Self::create))
            .route(&format!("{base_path}/{}", "{id}"), get(Self::retrieve))
            .route(&format!("{base_path}/{}", "{id}"), patch(Self::patch))
            .with_state(TicketStore::new());

        Self { router }
    }


    pub async fn serve(self, addr: impl ToSocketAddrs) -> Result<Serve<TcpListener, Router, Router>> {
        let listener = TcpListener::bind(addr).await?;
        Ok(axum::serve(listener, self.router))
    }

    async fn list_all(State(_): State<TicketStore>) -> Json<Vec<Ticket>> {
        todo!()
    }

    async fn create(State(_): State<TicketStore>) -> Json<TicketId> {
        todo!()
    }

    async fn retrieve(State(_): State<TicketStore>, Json(_): Json<TicketId>) -> Json<Ticket> {
        todo!()
    }

    async fn patch(State(_): State<TicketStore>, Json(_): Json<Value>) -> Json<Ticket> {
        todo!()
    }
}
