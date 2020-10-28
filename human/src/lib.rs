//! # Wire User & Group Management
//!
//! This crate implements user and group management.
//! 

use idem::Identity;
use mema::{Conversation, Message};

/// Errors that might occur in this crate.
#[derive(Debug)]
pub enum Error {}

/// Contact information about another user.
pub struct Contact {}

/// Group information.
pub struct Group {}

/// Authentication data such as BE token.
#[derive(Default)]
pub struct AuthData {}

/// The client represents a local user.
#[derive(Default)]
pub struct Client {
    contacts: Vec<Contact>,
    groups: Vec<Group>,
    conversations: Vec<Conversation>,
    auth_data: AuthData,
    identity: Identity,
}

impl Client {
    /// Login on the backend with username and password
    pub fn login(_username: String, _password: String) -> Self {
        Self::default()
    }

    /// Pull all messages from the server
    pub fn messages(&self) -> Result<Vec<Message>, Error> {
        todo!();
    }

    /// Send a message.
    pub fn send_message(&self, _msg: Message) -> Result<(), Error> {
        todo!();
    }
}
