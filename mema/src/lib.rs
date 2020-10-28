//! # Wire Message Management
//! 
//! This crate is responsible for handling messages.

/// A message.
#[derive(Debug)]
pub struct Message {}

/// A conversation is a list of messages.
/// It is also responsible to dynamically persist and load individual messages.
pub struct Conversation {
    messages: Vec<Message>,
}

/// There are a bunch of different message types.
pub enum MessageType {
    BackendMessage,
    MlsMessage,
}

impl Message {
    /// Get a new Message from bytes.
    pub fn from_bytes(_b: Vec<u8>) -> Self {
        todo!();
    }

    /// Encode the message to a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!();
    }
}

// XXX: not sure if we really want this.
impl From<&'static str> for Message {
    fn from(_s: &'static str) -> Self {
        todo!();
    }
}
