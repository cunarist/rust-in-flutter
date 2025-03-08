//! The actor model is highly recommended for state management,
//! as it provides modularity and scalability.
//! This module demonstrates how to use actors
//! within the async system in Rust.
//! To build a solid app, do not communicate by sharing memory;
//! instead, share memory by communicating.

use crate::common::*;
use crate::signals::*;
use messages::prelude::*;
use rinf::debug_print;

// TODO: Organize imports

// The letter type for communicating with an actor.
pub struct ClickedLetter;

// The actor that holds the counter state and handles messages.
pub struct CountingActor {
    // The counter number.
    count: i32,
}

// Implementing the `Actor` trait for `CountingActor`.
// This defines `CountingActor` as an actor in the async system.
impl Actor for CountingActor {}

impl CountingActor {
    pub fn new(counting_addr: Address<Self>) -> Self {
        spawn(Self::listen_to_button_click(counting_addr));
        CountingActor { count: 0 }
    }

    async fn listen_to_button_click(mut counting_addr: Address<Self>) {
        // Spawn an asynchronous task to listen for
        // button click signals from Dart.
        let receiver = SampleNumberInput::get_dart_signal_receiver();
        // Continuously listen for signals.
        while let Some(dart_signal) = receiver.recv().await {
            let letter = dart_signal.message.letter;
            debug_print!("{letter}");
            // Send a letter to the counting actor.
            let _ = counting_addr.send(ClickedLetter).await;
        }
    }
}

#[async_trait]
impl Handler<ClickedLetter> for CountingActor {
    type Result = ();
    // Handles messages received by the actor.
    async fn handle(&mut self, _msg: ClickedLetter, _context: &Context<Self>) {
        // Increase the counter number.
        let new_number = self.count + 7;
        self.count = new_number;

        // The send method is generated from a marked Protobuf message.
        SampleNumberOutput {
            current_number: new_number,
            dummy_one: 11,
            dummy_two: None,
            dummy_three: vec![22, 33, 44, 55],
        }
        .send_signal_to_dart();
    }
}

// Creates and spawns the actors in the async system.
pub async fn create_actors() -> Result<()> {
    // Create actor contexts.
    let counting_context = Context::new();
    let counting_addr = counting_context.address();

    // Spawn actors.
    let actor = CountingActor::new(counting_addr);
    spawn(counting_context.run(actor));

    Ok(())
}
