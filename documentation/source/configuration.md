# Configuration

## YAML File

You can customize the behavior of Rinf CLI commands by configuring the `pubspec.yaml` file. All fields are optional and it's not necessary to write them.

# TODO: Update the config

```{code-block} yaml
:caption: pubspec.yaml
rinf:
  message:
    input_dir: messages/
    rust_output_dir: native/hub/src/messages/
    dart_output_dir: lib/messages/
    rust_serde: true
```

You can check the current configuration status by running the command below in the CLI.

```{code-block} shell
:caption: CLI
rinf config
```

## Crate Features

Customizing the behavior of the Rinf crate is possible through its crate features.

```{code-block} toml
:caption: native/hub/Cargo.toml
rinf = { version = "0.0.0", features = ["feature-name"] }
```

- `show-backtrace`: Prints the full backtrace in the CLI when a panic occurs in debug mode. In general, backtrace is not very helpful when debugging async apps, so consider using [`tracing`](https://crates.io/crates/tracing) for logging purposes. Note that this feature does not affect debugging on the web platform.
- `bevy`: Implements the `Event` trait from `bevy_ecs` for `DartSignal`, allowing Bevy's entity component system to listen for events from Dart. This feature is highly experimental, and using it in production is not recommended.
