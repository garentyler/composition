use super::connection::Connection;
use crate::protocol::types::Chat;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::RwLock,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, trace};

pub type Callback = dyn Fn(u128, Arc<RwLock<Connection>>) + Send;

#[derive(Clone, Debug)]
pub struct NetworkListener {
    running: CancellationToken,
    clients: Arc<RwLock<HashMap<u128, Arc<RwLock<Connection>>>>>,
}
impl NetworkListener {
    pub async fn new<A: 'static + ToSocketAddrs + Send + std::fmt::Debug>(
        bind_address: A,
        running: CancellationToken,
        callback: Option<Box<Callback>>,
    ) -> Result<NetworkListener, std::io::Error> {
        let listener = TcpListener::bind(bind_address)
            .await
            .inspect_err(|_| error!("Could not bind to given address."))?;
        let clients = Arc::new(RwLock::new(HashMap::new()));

        let r = running.clone();
        let c = clients.clone();
        tokio::spawn(async move {
            trace!("Starting listener task");
            let mut client_id = 0u128;

            loop {
                tokio::select! {
                    _ = r.cancelled() => {
                        break;
                    }
                    result = listener.accept() => {
                        if let Ok((stream, _)) = result {
                            trace!("Listener task got connection (id {})", client_id);
                            let client = Arc::new(RwLock::new(Connection::new_client(client_id, stream)));
                            c.write().await.insert(client_id, client.clone());
                            if let Some(ref callback) = callback {
                                callback(client_id, client);
                            }
                            client_id += 1;
                        }
                    }
                }
            }
        });

        Ok(NetworkListener { running, clients })
    }
    pub async fn get_client(&self, id: u128) -> Option<Weak<RwLock<Connection>>> {
        self.clients.read().await.get(&id).map(Arc::downgrade)
    }
    pub async fn disconnect_client(
        &self,
        id: u128,
        reason: Option<Chat>,
    ) -> Result<Result<(), std::io::Error>, ()> {
        // Remove the client from the hashmap.
        let client = self.clients.write().await.remove(&id).ok_or(())?;
        let client: Connection = Arc::into_inner(client)
            .expect("only one reference")
            .into_inner();
        // let mut client = client.write().await;
        // Send a disconnect packet.
        Ok(client.disconnect(reason).await)
    }
    pub async fn shutdown(self, reason: Option<Chat>) -> Result<(), std::io::Error> {
        self.running.cancel();

        let reason = reason.unwrap_or(serde_json::json!({
            "text": "You have been disconnected!"
        }));

        let disconnections = self
            .clients
            .write()
            .await
            .drain()
            .map(|(_, c)| c)
            .map(|c| Arc::into_inner(c).expect("only one reference").into_inner())
            .map(|c| c.disconnect(Some(reason.clone())))
            .collect::<Vec<_>>();

        // We don't actually care if the disconnections succeed,
        // the connection is going to be dropped anyway.
        let _disconnections: Vec<Result<(), std::io::Error>> =
            futures::future::join_all(disconnections).await;

        Ok(())
    }
}
