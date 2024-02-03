//! Configuration that has to be the same between the server and the client.
use crate::client::config::NetcodeConfig;
use crate::client::resource::Authentication;
use crate::connection::client::ClientConnection;
use crate::prelude::IoConfig;
use bevy::utils::Duration;

use crate::shared::log::LogConfig;
use crate::shared::tick_manager::TickConfig;

#[derive(Clone, Debug)]
pub struct SharedConfig {
    pub enable_replication: bool,
    /// how often does the client send updates to the server?
    pub client_send_interval: Duration,
    /// how often does the server send updates to the client?
    pub server_send_interval: Duration,
    pub tick: TickConfig,
    pub log: LogConfig,
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self {
            enable_replication: false,
            // 0 means that we send updates every frame
            client_send_interval: Duration::from_millis(0),
            server_send_interval: Duration::from_millis(0),
            tick: TickConfig::new(Duration::from_millis(16)),
            log: LogConfig::default(),
        }
    }
}
