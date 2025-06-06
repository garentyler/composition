use crate::{
    net::{connection::DownstreamConnection, error::Error},
    protocol::{types::Chat, ClientState},
};
use std::{collections::HashMap, time::Duration};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::mpsc,
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, trace};

#[derive(Debug)]
pub struct DownstreamConnectionManager {
    max_clients: Option<usize>,
    clients: HashMap<u128, DownstreamConnection>,
    channel: (
        mpsc::UnboundedSender<DownstreamConnection>,
        mpsc::UnboundedReceiver<DownstreamConnection>,
    ),
}
impl DownstreamConnectionManager {
    pub fn new(max_clients: Option<usize>) -> DownstreamConnectionManager {
        DownstreamConnectionManager {
            max_clients,
            clients: HashMap::new(),
            channel: mpsc::unbounded_channel(),
        }
    }
    pub fn client(&self, id: u128) -> Option<&DownstreamConnection> {
        self.clients.get(&id)
    }
    pub fn client_mut(&mut self, id: u128) -> Option<&mut DownstreamConnection> {
        self.clients.get_mut(&id)
    }
    pub fn clients(&self) -> impl Iterator<Item = &DownstreamConnection> {
        self.clients.values()
    }
    pub fn clients_mut(&mut self) -> impl Iterator<Item = &mut DownstreamConnection> {
        self.clients.values_mut()
    }
    pub async fn spawn_listener<A>(
        &self,
        bind_address: A,
        running: CancellationToken,
    ) -> Result<JoinHandle<()>, Error>
    where
        A: 'static + ToSocketAddrs + Send + std::fmt::Debug,
    {
        trace!("Starting listener task");
        let fmt_addr = format!("{bind_address:?}");
        let listener = TcpListener::bind(bind_address)
            .await
            .map_err(Error::Io)
            .inspect_err(|_| error!("Could not bind to {}.", fmt_addr))?;

        let sender = self.channel.0.clone();

        let join_handle = tokio::spawn(async move {
            let mut client_id = 0u128;

            loop {
                tokio::select! {
                    _ = running.cancelled() => {
                        break;
                    }
                    result = listener.accept() => {
                        if let Ok((stream, _)) = result {
                            trace!("Listener task got connection (id {})", client_id);
                            let client = DownstreamConnection::new(client_id, stream);
                            if sender.send(client).is_err() {
                                trace!("Client receiver disconnected");
                                break;
                            }
                            client_id += 1;
                        }
                    }
                }
            }
            trace!("Listener task shutting down");
        });

        Ok(join_handle)
    }
    pub async fn update(&mut self) -> Result<(), Error> {
        // Receive new clients from the sender.
        loop {
            match self.channel.1.try_recv() {
                Ok(mut connection) => {
                    let id = connection.id();

                    match self.max_clients {
                        Some(max) => {
                            if self.clients.len() >= max {
                                let _ = connection.disconnect(None).await;
                            } else {
                                self.clients.insert(id, connection);
                            }
                        }
                        None => {
                            self.clients.insert(id, connection);
                        }
                    }
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    return Err(Error::ConnectionChannelDisconnnection)
                }
                Err(mpsc::error::TryRecvError::Empty) => break,
            };
        }

        // Disconnect any clients that have timed out.
        // We don't actually care if the disconnections succeed,
        // the connection is going to be dropped anyway.
        let _ = futures::future::join_all(self.clients.iter_mut().filter_map(|(_, client)| {
            if client.received_elapsed() > Duration::from_secs(10) {
                Some(client.disconnect(None))
            } else {
                None
            }
        }))
        .await;

        // Remove disconnected clients.
        let before = self.clients.len();
        self.clients
            .retain(|_id, c| c.client_state() != ClientState::Disconnected);
        let after = self.clients.len();
        if before - after > 0 {
            trace!("Removed {} disconnected clients", before - after);
        }

        Ok(())
    }
    pub async fn disconnect(
        &mut self,
        id: u128,
        reason: Option<Chat>,
    ) -> Option<Result<(), Error>> {
        let mut client = self.clients.remove(&id)?;
        Some(client.disconnect(reason).await)
    }
    pub async fn shutdown(mut self, reason: Option<Chat>) -> Result<(), Error> {
        let reason = reason.unwrap_or(serde_json::json!({
            "text": "You have been disconnected!"
        }));

        let mut clients = self.clients.drain().map(|(_, c)| c).collect::<Vec<_>>();
        let disconnections = clients
            .iter_mut()
            .map(|c| c.disconnect(Some(reason.clone())))
            .collect::<Vec<_>>();

        // We don't actually care if the disconnections succeed,
        // the connection is going to be dropped anyway.
        let _disconnections = futures::future::join_all(disconnections).await;

        Ok(())
    }
}
