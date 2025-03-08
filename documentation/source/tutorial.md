# Basic Tutorial

To grasp the basic concepts, it's beneficial to follow a step-by-step tutorial.

Before we start, make sure that there's a `Column` somewhere in your widget tree. This will contain our tutorial widgets.

```{code-block} dart
:caption: lib/main.dart
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [],
)
```

## From Dart to Rust

# TODO: Update the tutorial and messaging section

Let's say that you want to create a new button in Dart that sends an array of numbers and a string to Rust. We need a signal to notify Rust that a user event has occurred.

Write a new `.proto` file in the `./messages` directory with a new message. Note that the message should have the comment `[DART-SIGNAL]` above it.

```{code-block} proto
:caption: messages/tutorial_messages.proto
syntax = "proto3";
package tutorial_messages;

// [DART-SIGNAL]
message MyPreciousData {
  repeated int32 input_numbers = 1;
  string input_string = 2;
}
```

Next, generate Dart and Rust message code from `.proto` files.

```{code-block} shell
:caption: CLI
rinf gen
```

Create a button widget in Dart that accepts the user input.

```{code-block} dart
:caption: lib/main.dart
import 'package:test_app/messages/all.dart';

child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    ElevatedButton(
      onPressed: () async {
        MyPreciousData(
          inputNumbers: [3, 4, 5],
          inputString: 'Zero-cost abstraction',
        ).sendSignalToRust(); // GENERATED
      },
      child: Text('Send a Signal from Dart to Rust'),
    ),
  ]
)
```

Let's listen to this message in Rust. This simple function will add one to each element in the array and capitalize all letters in the string.

```{code-block} rust
:caption: native/hub/src/tutorial_functions.rs
use crate::common::*;
use crate::messages::*;
use rinf::debug_print;

pub async fn calculate_precious_data() {
    let receiver = MyPreciousData::get_dart_signal_receiver(); // GENERATED
    while let Some(dart_signal) = receiver.recv().await {
        let my_precious_data = dart_signal.message;

        let new_numbers: Vec<i32> = my_precious_data
            .input_numbers
            .into_iter()
            .map(|x| x + 1)
            .collect();
        let new_string = my_precious_data.input_string.to_uppercase();

        debug_print!("{new_numbers:?}");
        debug_print!("{new_string}");
    }
}
```

```{code-block} rust
:caption: native/hub/src/lib.rs
mod tutorial_functions;

#[tokio::main]
async fn main() {
    tokio::spawn(tutorial_functions::calculate_precious_data());
    rinf::dart_shutdown().await;
}
```

Now run the app with `flutter run`. We can see the printed output in the command-line when clicking the button!

```{code-block} none
:caption: Output
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

## From Rust to Dart

Let's say that you want to send increasing numbers every second from Rust to Dart.

Define the message. Note that the message should have the comment `[RUST-SIGNAL]` above it.

```{code-block} proto
:caption: messages/tutorial_messages.proto
syntax = "proto3";
package tutorial_messages;

// [RUST-SIGNAL]
message MyAmazingNumber { int32 current_number = 1; }
```

Generate Dart and Rust message code from `.proto` files.

```{code-block} shell
:caption: CLI
rinf gen
```

Define an async Rust function that runs forever, sending numbers to Dart every second.

```{code-block} toml
:caption: "native/hub/Cargo.toml"
tokio = { version = "1", features = ["sync", "rt", "time"] }
```

```{code-block} rust
:caption: native/hub/src/tutorial_functions.rs
use crate::messages::*;
use std::time::Duration;

pub async fn stream_amazing_number() {
    let mut current_number: i32 = 1;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        MyAmazingNumber { current_number }.send_signal_to_dart(); // GENERATED
        current_number += 1;
    }
}
```

```{code-block} rust
:caption: native/hub/src/lib.rs
mod tutorial_functions;

#[tokio::main]
async fn main() {
    tokio::spawn(tutorial_functions::stream_amazing_number());
    rinf::dart_shutdown().await;
}
```

Finally, receive the signals in Dart with `StreamBuilder` and rebuild the widget accordingly.

```{code-block} dart
:caption: lib/main.dart
import 'package:test_app/messages/all.dart';

children: [
  StreamBuilder(
    stream: MyAmazingNumber.rustSignalStream, // GENERATED
    builder: (context, snapshot) {
      final rustSignal = snapshot.data;
      if (rustSignal == null) {
        return Text('Nothing received yet');
      }
      final myAmazingNumber = rustSignal.message;
      final currentNumber = myAmazingNumber.currentNumber;
      return Text(currentNumber.toString());
    },
  ),
]
```

## Back and Forth

You can easily show the updated state on the screen by combining those two ways of message passing.

```{code-block} proto
:caption: messages/tutorial_messages.proto
syntax = "proto3";
package tutorial_messages;

// [DART-SIGNAL]
message MyTreasureInput {}

// [RUST-SIGNAL]
message MyTreasureOutput { int32 current_value = 1; }
```

```{code-block} shell
:caption: CLI
rinf gen
```

```{code-block} dart
:caption: lib/main.dart
import 'package:test_app/messages/all.dart';

children: [
  StreamBuilder(
    stream: MyTreasureOutput.rustSignalStream, // GENERATED
    builder: (context, snapshot) {
      final rustSignal = snapshot.data;
      if (rustSignal == null) {
        return Text('No value yet');
      }
      final myTreasureOutput = rustSignal.message;
      final currentNumber = myTreasureOutput.currentValue;
      return Text('Output value is $currentNumber');
    },
  ),
  ElevatedButton(
    onPressed: () {
      MyTreasureInput().sendSignalToRust(); // GENERATED
    },
    child: Text('Send the input'),
  ),
]
```

```{code-block} rust
:caption: native/hub/src/tutorial_functions.rs
use crate::common::*;
use crate::messages::*;

pub async fn tell_treasure() {
    let mut current_value: i32 = 1;

    let receiver = MyTreasureInput::get_dart_signal_receiver(); // GENERATED
    while let Some(_) = receiver.recv().await {
        MyTreasureOutput { current_value }.send_signal_to_dart(); // GENERATED
        current_value += 1;
    }
}
```

```{code-block} rust
:caption: native/hub/src/lib.rs
mod tutorial_functions;

#[tokio::main]
async fn main() {
    tokio::spawn(tutorial_functions::tell_treasure());
    rinf::dart_shutdown().await;
}
```
