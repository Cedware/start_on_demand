mod server;

use log::info;
use tokio::{io};
use crate::server::config::{Config, Mode};
use crate::server::container_manager::ContainerManager;
use crate::server::tcp::server::TcpServer;
use crate::server::udp::server::UdpServer;

#[tokio::main]
async fn main() -> io::Result<()> {

    env_logger::init();

    let config = Config::from_env();

    info!("starting with config: {:?}", config);

    let container_manager = ContainerManager::new(config.container_name, config.stop_container_timeout);

    match config.mode {
        Mode::Tcp => {
            let mut server = TcpServer::new(config.local_addr, config.remote_addr, config.disconnect_timeout, container_manager);
            server.start().await?
        },
        Mode::Udp => {
            let mut server = UdpServer::new(config.local_addr, config.remote_addr, config.disconnect_timeout, container_manager);
            server.start().await?
        }
    }
    Ok(())

}
