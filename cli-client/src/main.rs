//! # Wire CLI Client
//!
//! This is a very basic command line client for Wire based on the core library.
//! It is used for testing and is a blueprint for how to use th core library.

use nela::*;

/// The client providing high-level operations that call into the different parts
/// of the core library.
pub struct ClientApp {}
impl ClientApp {
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

fn main() {
    // Login on the backend.
    let username = String::from("franziskuscoretest");
    let password = String::from("");
    let client = ClientApp::login(username, password);

    // Register push notification listener.
    // This should obviously go into its own thread.
    fn listener(msg: Message) {
        println!("Got messages: {:?}", msg);

        // This is only a nudge to pull messages.
        // This requires changes to the callback.
    }
    client.register_notification_listener(listener);

    // Get messages
    let messages = match client.messages() {
        Ok(m) => m,
        Err(e) => {
            println!("Error retrieving messages: {:?}", e);
            std::process::exit(1);
        }
    };
    println!("Got messages: {:?}", messages);

    // Send a message.
    let msg = "Hi Fernando!";
    match client.send_message(Message::from(msg)) {
        Ok(()) => {}
        Err(e) => {
            println!("Error sending messages: {:?}", e);
            std::process::exit(1);
        }
    }
}
