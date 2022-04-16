use crate::prelude::*;
use tokio::net::ToSocketAddrs;

#[derive(Clone, Debug, PartialEq)]
pub enum ServerError {}

pub struct Server {}
impl Server {
    pub fn new<A: ToSocketAddrs>(bind_address: A) -> Server {
        unimplemented!()
    }
    pub async fn update(&mut self) -> Result<(), ServerError> {
        unimplemented!()
    }
    pub async fn shutdown(&mut self) -> Result<(), ServerError> {
        unimplemented!()
    }
}
