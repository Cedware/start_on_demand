use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::hash_map::HashMap;
use std::fmt::Display;
use std::io;
use std::sync::{Arc};
use std::sync::mpsc::Sender;
use std::time::Duration;
use log::info;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tokio::time::timeout;
use crate::server::container_manager::ContainerManager;
use crate::server::udp::client::UdpClientHandler;

pub struct UdpServer<A: ToSocketAddrs + Display + Clone + Send + Sync + 'static> {
    addr: A,
    remote_addr: A,
    disconnect_timeout: Duration,
    container_manager: Arc<ContainerManager>
}

impl<A: ToSocketAddrs + Display + Eq + Clone + Send + Sync + 'static> UdpServer<A> {
    pub fn new(addr: A, remote_addr: A, disconnect_timeout: Duration, container_manager: ContainerManager) -> Self {
        UdpServer {
            addr,
            remote_addr,
            disconnect_timeout,
            container_manager: Arc::new(container_manager),
        }
    }


    pub async fn start(&self) -> io::Result<()> {
        info!("Binding to {}", self.addr);
        let socket = Arc::new(UdpSocket::bind(&self.addr).await?);

        let channel_map = Arc::new(Mutex::new(HashMap::new()));
        let mut buffer = [0; 1024];
        loop {
            let (bytes_read, addr) = socket.recv_from(&mut buffer).await?;

            if let Vacant(e) = channel_map.lock().await.entry(addr) {
                self.container_manager.new_connection().await;
                let (sender, receiver) = channel(1024);
                let socket = socket.clone();
                let remote_addr = self.remote_addr.clone();
                let container_manager = self.container_manager.clone();
                let disconnect_timeout = self.disconnect_timeout;
                tokio::spawn(async move {
                    let mut client = UdpClientHandler::new(socket, remote_addr, addr, receiver, disconnect_timeout);
                    client.process().await.unwrap();
                    container_manager.connection_lost().await;
                });
                e.insert(sender);
            };
            let channel_map = channel_map.lock().await;
            let sender = channel_map.get(&addr).unwrap();
            sender.send((bytes_read, buffer)).await.unwrap();
        }
    }
}