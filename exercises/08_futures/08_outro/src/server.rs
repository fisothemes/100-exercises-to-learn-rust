use std::sync::Arc;
use tokio::net::{ToSocketAddrs, TcpListener};
use serde_json::Value;
use axum::{
    Router,
    Json,
    http::StatusCode,
    routing::get,
    serve::Serve,
    extract::{Path, State},
};
use axum::response::Html;
use tokio::sync::RwLock;
use crate::{
    error::{Result, Error},
    store::TicketStore,
    data::{TicketId, Ticket, TicketDraft},
};
use crate::data::{Status, TicketDescription, TicketTitle};

type Store = Arc<RwLock<TicketStore>>;

#[derive(Debug)]
pub struct Server;

impl Server {


    pub async fn serve(addr: impl ToSocketAddrs)
        -> Result<Serve<TcpListener, Router, Router>>
    {
        let store = Arc::new(RwLock::new(TicketStore::new()));
        
        let router = Router::new()
            .route("/", get(|| async { Html::from("Welcome to the ticket store!") }))
            .route("/tickets", get(Self::list_all).post(Self::create))
            .route("/tickets/{id}", get(Self::retrieve).patch(Self::patch))
            .with_state(store);
        
        let listener = TcpListener::bind(addr).await?;
        Ok(axum::serve(listener, router))
    }

    async fn list_all(State(store): State<Store>) -> Json<Vec<Ticket>> {
        let mut tickets = Vec::new();

        for ticket in store.read().await.get_all() {
            tickets.push(ticket.read().await.clone());
        }

        Json(tickets)
    }

    async fn create(State(store): State<Store>, Json(draft): Json<TicketDraft>)
        -> Json<TicketId> {
        Json(store.write().await.add_ticket(draft))
    }

    async fn retrieve(Path(id): Path<TicketId>, State(store): State<Store>)
        ->  Result<Json<Ticket>>
    {
        if let Some(ticket) = store.read().await.get(id) {
            Ok(Json(ticket.read().await.clone()))
        } else {
            Err(
                Error::HttpStatusCode(
                    StatusCode::NOT_FOUND, format!("Cannot find ticket with id: {id}.")
                )
            )
        }
    }

    async fn patch(Path(id): Path<TicketId>, State(store): State<Store>, Json(patch): Json<Value>)
                   -> Result<Json<Ticket>>
    {
        let Some(ticket) = store.read().await.get(id) else {
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

        if let Some(Value::String(title)) = map.get("title") {
            ticket.write().await.title = TicketTitle::try_from(title.to_string())?;
        }

        if let Some(Value::String(desc)) = map.get("description") {
            ticket.write().await.description = TicketDescription::try_from(desc.clone())?;
        }

        if let Some(Value::String(status)) = map.get("status") {
            ticket.write().await.status = Status::try_from(status.clone())?;
        }

        let ticket = ticket.read().await.clone();

        Ok(Json(ticket))
    }
}
