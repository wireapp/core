//! # Wire CLI Client
//!
//! This is a very basic command line client for Wire based on the core library.
//! It is used for testing and is a blueprint for how to use th core library.

use structopt::StructOpt;

use human::Client;
use mema::Message;
use nela::{Nela, ServerConfig};

#[derive(Debug)]
pub enum Error {}

/// The client providing high-level operations that call into the different parts
/// of the core library.
pub struct ClientApp {
    network: Nela,
    client: Client,
}
impl ClientApp {
    /// Login on the backend with username and password
    fn new(client: Client) -> Self {
        let client = Self {
            // XXX: make configurable.
            network: Nela::new(
                ServerConfig {
                    url: "https://prod-nginz-https.wire.com".to_string(),
                    port: 443,
                },
                ServerConfig {
                    url: "localhost".to_string(),
                    port: 8080,
                },
            ),
            client,
        };
        client.register_notification_listener();
        client
    }

    fn display(&self, conversation: String, limit: usize) {
        println!(
            "Displaying {:?} messages from conversation {:?}",
            limit, conversation
        );
    }

    /// Pull all new messages from the server and print them out.
    fn update(&mut self) -> Result<(), Error> {
        println!("TODO: Pull and print all messages");
        Ok(())
    }

    /// Send a message.
    fn send_message(&self, msg: String, conversation: String) -> Result<(), Error> {
        println!("TODO: Send message {:?} to {:?}", msg, conversation);
        Ok(())
    }

    /// Register a callback for push notifications.
    fn register_notification_listener(&self) {
        self.network.register_notification_listener(Self::listener);
    }

    // This should obviously go into its own thread.
    // There should also be a &self in here.
    fn listener(msg: Message) {
        println!("Got messages: {:?}", msg);

        // This is only a nudge to pull messages.
        // This requires changes to the callback.
    }
}

#[derive(Debug, StructOpt)]
struct CliOptions {
    username: String,
    password: String,
    operation: String,

    #[structopt(short = "c", long)]
    conversation: Option<String>,
    #[structopt(short = "l", long)]
    limit: Option<usize>,
    #[structopt(short = "m", long)]
    message: Option<String>,
}

fn main() {
    let opt = CliOptions::from_args();

    // Login on the backend.
    let mut client = ClientApp::new(Client::login(opt.username, opt.password));

    // Do the operation
    match opt.operation.as_str() {
        "get-all" => {
            // Get and print all new messages from all conversations.
            if client.update().is_err() {
                println!("Error retrieving messages.");
                std::process::exit(1);
            }
        }
        "get" => {
            // XXX: These should use subcommands.
            // Print "conversation"
            let conversation = match opt.conversation {
                Some(c) => c,
                None => {
                    println!("I need to know the conversation to display. Set --conversation.",);
                    std::process::exit(1);
                }
            };
            let limit = match opt.limit {
                Some(l) => l,
                None => 20, // Default limit.
            };
            client.display(conversation, limit);
        },
        "send" => {
            // Send a message to a conversation.
            let conversation = match opt.conversation {
                Some(c) => c,
                None => {
                    println!("I need where to send the message. Set --conversation.",);
                    std::process::exit(1);
                }
            };
            let msg = match opt.message {
                Some(m) => m,
                None => {
                    println!("I need a message to send. Set --message.",);
                    std::process::exit(1);
                }
            };
            client.send_message(msg, conversation).expect("Error sending msg");
        },
        _ => {
            println!("I don't know operation: {:?}", opt.operation);
            std::process::exit(1);
        }
    }
}
