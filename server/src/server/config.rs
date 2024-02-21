use std::env;
use std::fmt::Display;
use std::time::Duration;


#[derive(Debug)]
pub enum Mode {
    Tcp,
    Udp,
}

#[derive(Debug)]
pub struct Config {
    pub local_addr: String,
    pub remote_addr: String,
    pub disconnect_timeout: Duration,
    pub stop_container_timeout: Duration,
    pub mode: Mode,
    pub container_name: String,
    pub stop_container_on_start: bool
}

impl Config {
    pub fn from_env() -> Self {
        let local_addr = env::var("LOCAL_ADDR").unwrap();
        let remote_addr = env::var("REMOTE_ADDR").unwrap();
        let disconnect_timeout = env::var("DISCONNECT_TIMEOUT").unwrap().parse::<u64>().unwrap();
        let stop_container_timeout = env::var("STOP_CONTAINER_TIMEOUT").unwrap().parse::<u64>().unwrap();
        let container_name = env::var("CONTAINER_NAME").unwrap();
        let stop_container_on_start = env::var("STOP_CONTAINER_ON_START").unwrap().parse::<bool>().unwrap();

        let mode = match env::var("MODE").unwrap().as_str() {
            "tcp" => Mode::Tcp,
            "udp" => Mode::Udp,
            _ => panic!("unknown mode")
        };

        Config {
            local_addr,
            remote_addr,
            mode,
            container_name,
            stop_container_on_start,
            disconnect_timeout: Duration::from_secs(disconnect_timeout),
            stop_container_timeout: Duration::from_secs(stop_container_timeout)
        }
    }
}