use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;
use log::info;
use tokio::io;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::sync::Mutex;
use crate::server::container_manager::ContainerManager;
use crate::server::tcp::client::TcpClientHandler;

pub struct TcpServer<A: ToSocketAddrs + Send + Display + Clone + Sync + 'static> {
    addr: A,
    remote_addr: A,
    disconnect_timeout: Duration,
    container_manager: Arc<ContainerManager>
}

impl <A: ToSocketAddrs+ Send + Display + Clone + Sync + 'static> TcpServer<A> {

    pub fn new(addr: A, remote_addr: A, disconnect_timeout: Duration, container_manager: ContainerManager) -> Self {
        TcpServer {
            addr,
            remote_addr,
            disconnect_timeout,
            container_manager: Arc::new(container_manager)
        }
    }

    pub async fn start(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            info!("Received connection from {}", addr);
            self.container_manager.new_connection().await;


            let mut handler = TcpClientHandler::new(stream, self.remote_addr.clone(), self.disconnect_timeout);

            let container_manager = self.container_manager.clone();
            tokio::spawn(async move {
                handler.process().await.unwrap();
                container_manager.connection_lost().await;
            });


        }
    }


}
