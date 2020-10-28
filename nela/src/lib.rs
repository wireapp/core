//! # Wire Networking Layer
//!

use mema::Message;

/// Errors that might occur on the networking layer.
#[derive(Debug)]
pub enum Error {}

// https://github.com/wearezeta/backend-api-docs/wiki/API-User-Authentication#password-login
pub struct LoginMessage {
    email: String,
    password: String,
}

/// Function pointer to retrieve push notifications from the server.
pub type PushNotificationListener = fn(Message);

/// Server configuration.
pub struct ServerConfig {
    pub url: String,
    pub port: u16,
}

/// The main entry point for networking operations holding the configuration.
pub struct Nela {
    // https://prod-nginz-https.wire.com
    wire_backend: ServerConfig,
    mls_backend: ServerConfig,
}

impl Nela {
    /// Instantiate a new network configuration.
    pub fn new(wire_backend: ServerConfig, mls_backend: ServerConfig) -> Self {
        Self {
            wire_backend,
            mls_backend,
        }
    }

    /// Send a message to the backend server.
    pub fn send_be(&self, _data: Vec<u8>) -> Result<(), Error> {
        todo!();
    }

    /// Send a message to the delivery server.
    pub fn send_mls(&self, _data: Vec<u8>) -> Result<(), Error> {
        todo!();
    }

    /// Receive message from the backend server.
    pub fn receive_be(&self, _data: Vec<u8>) -> Result<(), Error> {
        todo!();
    }

    /// Receive messages from the delivery server.
    pub fn receive_mls(&self, _data: Vec<u8>) -> Result<(), Error> {
        todo!();
    }

    /// Register a callback for push notifications.
    pub fn register_notification_listener(&self, _listener: PushNotificationListener) {
        // todo!();
    }
}
