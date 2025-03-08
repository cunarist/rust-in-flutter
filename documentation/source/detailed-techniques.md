# Detailed Techniques

## Signal Members

We've covered how to pass signals[^1] between Dart and Rust in the previous tutorial section. Now Let's delve into the meaning of each field of a signal.

[^1]: Rinf relies solely on native FFI for communication, avoiding the use of web protocols or hidden threads. The goal is to minimize performance overhead as much as possible.

- **Field `message`:** It represents a message of a type defined by Protobuf. This field is always filled.

- **Field `binary`:** This is a field designed to handle large binary data, potentially up to a few gigabytes. You can send any kind of binary data you wish, such as a high-resolution image or file data. This field carries empty `Uint8List` or `Vec<u8>` if the message is not marked as binary signal.

It's important to note that creating a Protobuf `message` larger than a few megabytes is not recommended. For large data, split them into multiple signals, or use the `binary` field instead.[^2]

[^2]: Sending a serialized message or binary data is a zero-copy operation from Rust to Dart, while it involves a copy operation from Dart to Rust in memory. Keep in mind that Protobuf's serialization and deserialization does involve memory copy.

## Generation Path

When you generate message code using the `rinf gen` command, the resulting Dart modules will be placed under `lib/src/generated` folder.

## Continuous Watching

If you add the optional argument `-w` or `--watch` to the `rinf gen` command, the message code will be automatically generated when Rust files are modified. If you add this argument, the command will not exit on its own.

```{code-block} shell
:caption: CLI
rinf gen --watch
```
