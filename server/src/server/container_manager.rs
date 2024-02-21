use std::sync::Arc;
use std::time::Duration;
use log::info;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::server::docker::{start_container, stop_container};

pub struct ContainerManager {
    connection_count: Arc<Mutex<u32>>,
    container_name: String,
    stop_timeout: Duration
}

impl ContainerManager {

    pub fn new(container_name: String, stop_timeout: Duration) -> Self {
        ContainerManager {
            connection_count: Arc::new(Mutex::new(0)),
            container_name,
            stop_timeout
        }
    }



    pub async fn new_connection(&self) {
        *self.connection_count.lock().await += 1;
        info!("Registered new connection, connection count: {}", self.connection_count.lock().await);
        start_container(&self.container_name).await.unwrap();
    }

    pub async fn connection_lost(&self) {
        *self.connection_count.lock().await -= 1;
        info!("Lost client, connection count: {}", self.connection_count.lock().await);
        info!("Waiting 60s before shutting down container");

        let connection_count = self.connection_count.clone();
        let container_name = self.container_name.clone();
        let stop_timeout = self.stop_timeout;
        tokio::spawn(async move {
            sleep(stop_timeout).await;
            if *connection_count.lock().await == 0 {
                info!("Stopping container");
                stop_container(&container_name).await.unwrap();
            }
            else {
                info!("New clients connected, not stopping container")
            }
        });

    }

}