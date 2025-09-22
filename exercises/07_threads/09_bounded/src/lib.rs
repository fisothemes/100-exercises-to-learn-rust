// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, RecvError, SendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TicketStoreClientError> {
        let (response_channel, rx) = sync_channel(1);

        if let Err(err) = self.sender.send(Command::Insert {draft, response_channel}) {
            return Err(TicketStoreClientError::SendError(err));
        }

        match rx.recv() {
            Err(err) => Err(TicketStoreClientError::RecvError(err)),
            Ok(id) => Ok(id)
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TicketStoreClientError> {
        let (response_channel, rx) = sync_channel(1);

        if let Err(err) = self.sender.send(Command::Get {id, response_channel}){
            return Err(TicketStoreClientError::SendError(err))
        }

        match rx.recv() {
            Err(err) => Err(TicketStoreClientError::RecvError(err)),
            Ok(ticket) => Ok(ticket)
        }

    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient{ sender }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketStoreClientError {
    #[error("Failed to send command!")]
    SendError(#[from] SendError<Command>),
    #[error("Failed to recv response from client!")]
    RecvError(#[from] RecvError)
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let _ = response_channel.send(store.add_ticket(draft));
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let _ = response_channel.send(store.get(id).cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
