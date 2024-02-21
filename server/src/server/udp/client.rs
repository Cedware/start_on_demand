use std::fmt::Debug;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use log::info;
use tokio::io::join;
use tokio::join;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::mpsc::Receiver;
use tokio::time::timeout;

pub struct UdpClientHandler<A: ToSocketAddrs + Send + Clone + 'static> {
    client_side_socket: Arc<UdpSocket>,
    remote_addr: A,
    client_addr: SocketAddr,
    data_receiver: Receiver<(usize, [u8; 1024])>,
    disconnect_timeout: Duration
}

impl <A: ToSocketAddrs + Send + Clone + 'static> UdpClientHandler<A> {


    pub fn new(client_side_socket: Arc<UdpSocket>, remote_addr: A, client_addr: SocketAddr, data_receiver: Receiver<(usize, [u8; 1024])>, disconnect_timeout: Duration) -> Self {
        UdpClientHandler {
            client_side_socket,
            remote_addr,
            client_addr,
            data_receiver,
            disconnect_timeout
        }
    }

    async fn forward_data_to_remote(receiver: &mut Receiver<(usize, [u8; 1024])>, socket: &UdpSocket, remote_addr: A, receive_timeout: Duration) -> io::Result<()> {
        info!("starting forward_data_to_remote");
        loop {
            let (bytes_received, buffer) = match timeout(receive_timeout, receiver.recv()).await {
                Ok(Some(data )) => data,
                _ => {
                    info!("Timeout or received none");
                    (0,[0; 1024])
                }
            };
            if bytes_received == 0 {
                info!("Received 0 bytes, stopping to forward data");
                break;
            }
            socket.send_to(&buffer[0..bytes_received], &remote_addr).await?;

        }
        Ok(())
    }

    async fn forward_data_to_client(remote_socket: &UdpSocket, client_socket: &UdpSocket, client_addr: &SocketAddr, receive_timeout: Duration) -> io::Result<()> {
        info!("starting forward_data_to_client");
        let mut buffer = [0; 1024];
        loop {
            let bytes_received = match timeout(receive_timeout, remote_socket.recv_from(&mut buffer)).await? {
                Ok((length, _) ) => length,
                _ => {
                    info!("Timeout or received none");
                    0
                }
            };
            if bytes_received == 0 {
                info!("Received 0 bytes, stopping to forward data");
                break;
            }
            client_socket.send_to(&buffer[0..bytes_received], client_addr).await?;
        }
        Ok(())
    }

    pub async fn process(&mut self) -> io::Result<()> {
        info!("Opening remote side socket");
        let udp_socket = UdpSocket::bind("0.0.0.0:0").await?;
        let receive_data = &mut self.data_receiver;

        let (_,_ ) = join!(
            Self::forward_data_to_remote(receive_data, &udp_socket, self.remote_addr.clone(), self.disconnect_timeout),
            Self::forward_data_to_client(&udp_socket, &self.client_side_socket, &self.client_addr, self.disconnect_timeout)
        );


        Ok(())
    }

}
