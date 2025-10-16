use tokio::net::{ToSocketAddrs, TcpListener};
use serde_json::{Value, Map};
use axum::{
    Router,
    Json,
    routing::{get, post, patch},
    serve::Serve,
    extract::{Path, State},
};
use crate::{
    error::Result,
    store::TicketStore,
    data::{TicketId, Ticket, TicketDraft},
};
use crate::data::{Status, TicketDescription, TicketTitle};

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

    async fn list_all(State(store): State<TicketStore>) -> Json<Vec<Ticket>> {
        let mut tickets = Vec::new();

        for ticket in store.get_all() {
            tickets.push(ticket.read().await.clone());
        }

        Json(tickets)
    }

    async fn create(State(mut store): State<TicketStore>, Json(draft): Json<TicketDraft>)
        -> Json<TicketId>
    {
        Json(store.add_ticket(draft))
    }

    async fn retrieve(Path(id): Path<TicketId>, State(store): State<TicketStore>)
        -> Json<Option<Ticket>>
    {
        if let Some(ticket) = store.get(id) {
            Json(Some(ticket.read().await.clone()))
        } else {
            Json(None)
        }
    }

    async fn patch(Path(id): Path<TicketId>, State(mut store): State<TicketStore>, Json(patch): Json<Value>)
        -> Json<Option<Ticket>>
    {
        if let Some(ticket) = store.get_mut(id){
            match patch {
                Value::Object(map) => {
                    if let Some(title) = map.get("title") {
                        let title = serde_json::to_string(title).unwrap();
                        ticket.write().await.title = TicketTitle::try_from(title).unwrap();
                    }

                    if let Some(desc) = map.get("description") {
                        let desc = serde_json::to_string(desc).unwrap();
                        ticket.write().await.description = TicketDescription::try_from(desc).unwrap();
                    }

                    if let Some(status) = map.get("status") {
                        let status = serde_json::to_string(status).unwrap();
                        ticket.write().await.status = Status::try_from(status).unwrap();
                    }
                    
                    Json(Some(ticket.read().await.clone()))
                }
                _ => Json(None)
            }
        } else {
            Json(None)
        }

    }
}
