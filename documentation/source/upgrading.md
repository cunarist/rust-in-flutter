# Upgrading Rinf

## Minor Upgrades

Whenever upgrading Rinf, please ensure that the Rinf versions in `pubspec.yaml` and `native/hub/Cargo.toml` are identical.

## Migrating from 6 to 7

The overall usage remains the same, but some changes have been made to the API to improve code readability and flexibility.

Explicitly bind the `main` function in Rust with the async runtime of your choice. Also, don't forget to await the `dart_shutdown` future provided by Rinf in the `main` function.

```{code-block} rust
:caption: Rust
[tokio::main]
async fn main() {
    // Do whatever you want here.
    rinf::dart_shutdown().await;
}
```

Remove `RINF:` from Protobuf message annotations. For example, `[RINF:DART-SIGNAL]` should become `[DART-SIGNAL]`.

```{code-block} proto
:caption: Protobuf
// [DART-SIGNAL]
message SomeMessage {}
```

Import messages from the root `generated` module, not from the inner module where they are declared.

```{code-block} dart
:caption: Dart
import 'generated.dart';
```

```{code-block} rust
:caption: Rust
use crate::messages::*;
```

When you need to run a Flutter web server, use `rinf server` to get the complete Flutter command with the necessary arguments.

```{code-block} shell
:caption: CLI
rinf server
```

## Migrating from 7 to 8

# TODO: Fill these in

- Recursive signal derive is needed
