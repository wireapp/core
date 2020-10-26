//! # Wire User & Group Management
//!
//! This crate implements user and group management.

use idem::Identity;

/// Contact information about another user.
pub struct Contact {}

/// Group information.
pub struct Group {}

/// Authentication data such as BE token.
pub struct AuthData {}

/// The client represents a local user.
pub struct Client {
    contacts: Vec<Contact>,
    groups: Vec<Group>,
    auth_data: AuthData,
    identity: Identity,
}

impl Client {
    /// Login on the backend with username and password
    pub fn login(_username: String, _password: String) -> Self {
        todo!();
    }

    /// Pull all messages from the server
    pub fn messages(&self) -> Result<Vec<Message>, Error> {
        todo!();
    }

    /// Send a message.
    pub fn send_message(&self, _msg: Message) -> Result<(), Error> {
        todo!();
    }

    /// Register a callback for push notifications.
    pub fn register_notification_listener(&self, _listener: PushNotificationListener) {
        todo!();
    }
}
