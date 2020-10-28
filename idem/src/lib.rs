//! # Wire Identity Management
//!
//! This crate handles user and client identities.
//!
//! For now only basic credentials are supported, replicating the existing keys
//! in the legacy Wire applications.

#[derive(Default)]
pub struct Identity {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
}

impl Identity {
    /// Create a new random identity.
    pub fn new() -> Self {
        todo!();
    }

    /// Get the raw keys (private, public).
    pub fn to_raw(&self) -> (Vec<u8>, Vec<u8>) {
        (self.private_key.clone(), self.public_key.clone())
    }
}
