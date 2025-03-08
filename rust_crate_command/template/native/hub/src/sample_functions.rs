//! This module is written for Rinf demonstrations.

use crate::{SmallNumber, SmallText};

pub async fn communicate() {
    // Send signals to Dart like below.
    SmallNumber { number: 7 }.send_signal_to_dart();

    // Get receivers that listen to Dart signals like below.
    let receiver = SmallText::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let message: SmallText = dart_signal.message;
        rinf::debug_print!("{message:?}");
    }
}

// Though async tasks work, using the actor model
// is highly recommended for state management
// to achieve modularity and scalability in your app.
// To understand how to use the actor model,
// refer to the Rinf documentation.
