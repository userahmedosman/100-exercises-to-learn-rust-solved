
use std::sync::mpsc;
// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<TicketDraft>> {

        let (response_sender, response_receiver) = mpsc::channel();
        let id = self.sender.send(Command::Insert {
            draft:draft.clone(),
            response_channel: response_sender,
        }).unwrap();

        match response_receiver.recv(){
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(TrySendError::Full(draft))
            }
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<TicketId>> {

        let (response_sender, response_receiver) = mpsc::channel();

        let ticket = self.sender.send(Command::Get {
            id,
            response_channel: response_sender,
        }).unwrap();

        match response_receiver.recv(){
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(TrySendError::Full(id))
            }
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (tx, rx) = mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(rx));
    TicketStoreClient { sender: tx }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
