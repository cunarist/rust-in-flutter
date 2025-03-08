# FAQ

## Question Lists

### Where should I ask for help?

If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/rinf/discussions) and open a Q&A thread for assistance.

### Where should I use Rust?

Ideally, **Flutter** would deal with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end can be completely separated, meaning that Dart and Rust codes are detachable from each other.

### How are data passed between Dart and Rust?

Data being sent between Dart and Rust are basically bytes arrays, represented as `Uint8List` in Dart and `Vec<u8>` in Rust. You can send Protobuf messages and any binary data as you wish such as a high-resolution image or some kind of file data.

### Where are the library files generated from Rust crates?

All build settings of Rinf ensures that all library files compiled from Rust crates are properly included in the final build, ready to be distributed. Therefore you do not need to worry about bundling library files.

### Android app build has failed. What should I do?

The NDK version that your project expects is specified in `./android/app/build.gradle` file as `ndkVersion` variable inside `android` block. The value of this `ndkVersion` should be `flutter.ndkVersion` and you should be using Flutter SDK [3.10 or higher](https://docs.flutter.dev/release/release-notes/release-notes-3.10.0). However, `ndkVersion` can be absent if you've created your Flutter project with Flutter SDK 3.7 and earlier. If `ndkVersion` is not defined in your `./android/app/build.gradle` file, go ahead and write one yourself.

```{code-block} none
:caption: android/app/build.gradle
..
android {
    namespace "com.cunarist.rinf_example"
    compileSdkVersion flutter.compileSdkVersion
    ndkVersion flutter.ndkVersion // <------ THIS LINE

    compileOptions {
        sourceCompatibility JavaVersion.VERSION_1_8
        targetCompatibility JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = '1.8'
    }

    sourceSets {
        main.java.srcDirs += 'src/main/kotlin'
    }
..
```

### How does concurrency work under the hood?

On native platforms, Dart runs in the main thread, while Rust utilizes the async `tokio` runtime, allowing async tasks to run efficiently within a separate thread.

On the web, Dart and Rust both run inside JavaScript's async event loop in the main thread, with Rust `Future`s being converted into JavaScript `Promise`s internally. This is a necessary constraint because [webassembly component proposal](https://github.com/WebAssembly/proposals) is not stabilized as of February 2024.

### Will changes made to Rust code take effect upon Dart's hot restart?

No, the updated Rust code cannot be loaded upon Dart's hot restart. To incorporate the changes, the app needs to be re-compiled, as the app binary must be linked to the newly compiled Rust library files again. This limitation arises from the Rust compilation process, as Rust does not inherently support a hot restart feature.

On native platforms, Dart's hot restart makes the Rust logic restart, in other words, the `async fn main()` function. On the web, Dart's hot restart has no effect on the Rust logic, because it's not possible to cancel all the async tasks that are already queued inside the JavaScript event loop.

### How do I use nightly Rust?

In order to use nightly Rust, you need to add a cargokit configuration file. Cargokit is the build connector between Dart and Rust used by this framework.

```{code-block} yaml
:caption: native/hub/cargokit.yaml
cargo:
  debug:
    toolchain: nightly
  release:
    toolchain: nightly
```

More information about `cargokit.yaml` can be found at the link below. Cargokit is the linker for Rust crates that are used in various Flutter projects, including Rinf.

- https://github.com/irondash/cargokit/blob/main/docs/architecture.md

### Is it safe enough to pass secret parameters between Dart and Rust?

It is safe to pass secret parameters between Dart and Rust. Some other Rust GUI frameworks use HTTP or websockets to communicate between GUI and Rust, which is quite insecure. However, that's not the case for Rinf because messages are passed within the Flutter app's process. Please note that while it is hard to reverse-engineer compiled native binaries to search for secret keys or params, it is generally not recommended to hardcode sensitive information in the application itself. This caution applies to this framework as well as any other distributed binaries.

### Can I use this in pure Dart projects?

No, this framework only supports GUI Flutter apps because it's basically a 'Flutter FFI plugin'. This framework does NOT support other types of projects:

- Flutter GUI app: **Mainly supported ☀️**
- Flutter plugin: Not supported, but this package can be an inspiration
- Dart CLI app: Might not work, this framework has to hook onto the Flutter SDK
- Dart package: Not supported, unable to hook onto Dart SDK builds

However, we also promise to deliver the best development experience as it is mainly focused on this **one** category.

### What happens when a panic occurs in Rust?

A Rust panic doesn't crash the app; it simply cancels the spawned async task. You don't need to worry about app stability due to Rust panics. When a panic occurs, the information will be displayed in the CLI if the app is running in debug mode.

> On the web, unfortunately, Rust panic is not propageted up the stack because of [this limitation](https://github.com/rustwasm/wasm-bindgen/issues/2724) in `wasm-bindgen` as of January 2024.

### How do I make Rust-analyzer lint in webassembly mode?

There might be various Rust codes with these attribute above:

```{code-block} rust
:caption: Rust
#[cfg(target_family = "wasm")]
{}
#[cfg(not(target_family = "wasm"))]
{}
```

Since the environments of the web and native platforms are so different, there are times when you need to use these attributes to include and exclude parts of the code depending on whether they are targeting web or not.

By default, Rust-analyzer runs in native mode. To make it run in webassembly mode, create the configuration file:

```{code-block} toml
:caption: .cargo/config.toml
[build]
# Uncomment the line below to switch Rust-analyzer to perform
# type checking and linting in webassembly mode, for the web target.
# You might have to restart Rust-analyzer for this change to take effect.
target = "wasm32-unknown-unknown"
```

You need to restart Rust language server for this to take effect.

### CMake cache is broken after I moved the app folder

```{code-block} none
:caption: Output
CMake Error: The current CMakeCache.txt directory C:/.../CMakeCache.txt is different than the directory C:/... where CMakeCache.txt was created. This may result in binaries being created in the wrong place. If you are not sure, reedit the CMakeCache.txt
CMake Error: The source "C:/.../CMakeLists.txt" does not match the source "C:/.../CMakeLists.txt" used to generate cache.  Re-run cmake with a different source directory.
Building Windows application...                                     80ms
Exception: Unable to generate build files
```

This error can simply be fixed with the command below.

```{code-block} shell
:caption: CLI
flutter clean
cargo clean
```

### I encountered an error related to loading native libraries on older Android versions.

If you are using older Android versions, you may encounter errors due to issues with native library loading.

To address this, you can modify `AndroidManifest.xml` files under `./android/app/src/` as follows.

```xml
:caption: android/app/src/**/AndroidManifest.xml
<application
    android:extractNativeLibs="true"
>
```

### Some of the standard library modules don't work on the web

As of February 2024, Rinf utilizes the `wasm32-unknown-unknown` Rust target for web development. However, this target has certain limitations, particularly in terms of system IO capabilities. The hope is to [transition](https://github.com/cunarist/rinf/issues/204) to `wasm32-wasi` in the future pending the stabilization of the [WebAssembly component proposal](https://github.com/WebAssembly/proposals).

Here are the current constraints of the `wasm32-unknown-unknown` target:

- Numerous functionalities within `std::fs` remain unimplemented.
- Various features of `std::net` are not available. Consider using `reqwest` crate instead. `reqwest` supports `wasm32-unknown-unknown` and relies on JavaScript to perform network communications.
- `std::thread::spawn` doesn't work. Consider using `tokio_with_wasm::task::spawn_blocking` instead.
- Several features of `std::time::Instant` are unimplemented. Consider using `chrono` as an alternative. `chrono` supports `wasm32-unknown-unknown` and relies on JavaScript to obtain system time.
- In case of a panic in an asynchronous Rust task, it aborts and throws a JavaScript `RuntimeError` [which Rust cannot catch](https://stackoverflow.com/questions/59426545/rust-paniccatch-unwind-no-use-in-webassembly). A recommended practice is to handle errors with `Err` instances.

### My app failed to load dynamic library

```{code-block} none
:caption: Output
Exception has occurred.
ArgumentError (Invalid argument(s): Failed to load dynamic library 'libhub.so': dlopen failed: cannot locate symbol "..." referenced by ...
```

This happens when one or some of your Rust dependencies expect to have C or C++ libraries linked to the `hub` crate. Not all Rust crates on `crates.io` are written in pure Rust, and some depends on C code with `libc++`, `libstdc++`, etc.

To make `cargo` link those C or C++ libraries to your native library, create your `build.rs` file like below.

```{code-block} rust
:caption: native/hub/build.rs
use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("android") => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=c++_shared");
        },
        _ => {}
    }
}
```

The code above describes how to link `libc++` to your Android app. You can modify this code to adapt to certain scenarios.

These links might be a help:

- https://github.com/cunarist/rinf/issues/280
- https://kazlauskas.me/entries/writing-proper-buildrs-scripts
- https://github.com/RustAudio/rodio/issues/404
- https://github.com/breez/c-breez/issues/553

### How do I set the path to a compiled dynamic library?

You might want to run your app on embedded devices. However, you may encounter this error when running your app on a non-major platform:

```
Failed to load dynamic library 'libhub.so': libhub.so: cannot open shared object file: No such file or directory
```

In this case, you can specify a path that points to the compiled Rust library. Simply provide a string path to your dynamic library file.

```{code-block} dart
:caption: lib/main.dart
import 'generated.dart';

async void main() {
  await initializeRust(compiledLibPath: '/path/to/library/libhub.so');
}
```

This provided path will be used for finding dynamic library files on native platforms with Dart's `DynamicLibrary.open([compiledLibPath])`, and for loading the JavaScript module on the web with `import init, * as wasmBindings from "[compiledLibPath]"`.
