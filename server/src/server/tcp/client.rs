use std::fmt::Display;
use std::time::Duration;
use log::{debug, info};
use tokio::{io, join};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::time::timeout;


pub struct TcpClientHandler<A: ToSocketAddrs> {
    stream: TcpStream,
    remote_addr: A,
    disconnect_timeout: Duration
}

impl<A: ToSocketAddrs + Display> TcpClientHandler<A> {
    pub fn new(stream: TcpStream, remote_addr: A, disconnect_timeout: Duration) -> Self {
        TcpClientHandler {
            stream,
            remote_addr,
            disconnect_timeout
        }
    }

    async fn connect_to_remote(&self) -> io::Result<TcpStream> {
        info!("connecting to {}", &self.remote_addr);
        TcpStream::connect(&self.remote_addr).await
    }

    async fn copy_data<R: AsyncReadExt + Unpin, W: AsyncWriteExt + Unpin>(mut from: R, mut to: W, disconnect_timeout: Duration) -> io::Result<()> {
        info!("copying data");
        let mut buffer = [0u8; 32768];
        loop {
            let bytes_read = match timeout(disconnect_timeout, from.read(&mut buffer)).await {
                Ok(Ok(bytes_read)) => bytes_read,
                _ => {
                    info!("Didn't receive data for 10 seconds, considering connection closed");
                    0
                }
            };
            to.write_all(&buffer[0..bytes_read]).await?;
            if bytes_read == 0 {
                info!("Received 0 bytes, stopping read loop");
                break;
            }

        }
        Ok(())
    }

    pub async fn process(&mut self) -> io::Result<()> {
        info!("connecting to remote");
        let mut remote_stream = TcpStream::connect(&self.remote_addr).await?;



        debug!("Splitting remote stream into read and write halfs");
        let (remote_read, remote_write) = remote_stream.split();
        debug!("Splitting client stream into read and write halfs");
        let (client_read, client_write) = self.stream.split();


        let (remote_to_client, client_to_remote) = join!(
            Self::copy_data(client_read, remote_write, self.disconnect_timeout),
            Self::copy_data(remote_read, client_write, self.disconnect_timeout),
        );

        remote_to_client?;
        client_to_remote?;

        info!("Finished copying data");

        Ok(())
    }
}