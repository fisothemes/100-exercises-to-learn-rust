use tokio::net::{ToSocketAddrs, TcpListener};
use serde_json::Value;
use axum::{
    Router,
    Json,
    http::StatusCode,
    routing::{get, post, patch},
    serve::Serve,
    extract::{Path, State},
};
use axum::response::Html;
use crate::{
    error::{Result, Error},
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
            .route("/", get(|| async { Html::from("Welcome!") }))
            .route(base_path, get(Self::list_all))
            .route(base_path, post(Self::create))
            .route(&format!("{base_path}/{{id}}"), get(Self::retrieve))
            .route(&format!("{base_path}/{{id}}"), patch(Self::patch))
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
        -> Json<TicketId> { Json(store.add_ticket(draft)) }

    async fn retrieve(Path(id): Path<TicketId>, State(store): State<TicketStore>)
        ->  Result<Json<Ticket>>
    {
        if let Some(ticket) = store.get(id) {
            Ok(Json(ticket.read().await.clone()))
        } else {
            Err(
                Error::HttpStatusCode(
                    StatusCode::NOT_FOUND, format!("Cannot find ticket with id: {id}.")
                )
            )
        }
    }

    async fn patch(Path(id): Path<TicketId>, State(mut store): State<TicketStore>, Json(patch): Json<Value>)
        -> Result<Json<Ticket>>
    {
        let Some(ticket) = store.get_mut(id) else {
            return Err(
                Error::HttpStatusCode(
                    StatusCode::NOT_FOUND, format!("Cannot find ticket with id: {id}.")
                )
            )
        };

        let Value::Object(map) = serde_json::to_value(patch)? else {
            return Err(
                Error::JsonParse(serde::de::Error::custom("Invalid type. Expected JSON object."))
            )
        };

        if let Some(title) = map.get("title") {
            ticket.write().await.title = TicketTitle::try_from(title.to_string())?;
        }

        if let Some(desc) = map.get("description") {
            ticket.write().await.description = TicketDescription::try_from(desc.to_string())?;
        }

        if let Some(status) = map.get("status") {
            ticket.write().await.status = Status::try_from(status.to_string())?;
        }

        Ok(Json(ticket.read().await.clone()))
    }
}
