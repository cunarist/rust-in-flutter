# Applying Rust Template

```{note}
This section assumes that you've already created a Flutter project with `flutter create [my_app_name]`. You can get further guidance from [this awesome Flutter tutorial](https://docs.flutter.dev/get-started/codelab).
```

First of all, add this framework to your Flutter project.

```{code-block} shell
:caption: CLI
flutter pub add rinf
```

Now install the command executable to easily run Rinf commands in the CLI.[^1]

[^1]: If you're curious about all the available commands, use `rinf --help`.

```{code-block} shell
:caption: CLI
cargo install rinf
```

Then, simply run this in the command-line from your Flutter project's directory.

```{code-block} shell
:caption: CLI
rinf template
```

After running the command, you'll have new files and folders as your starter Rust template.

```{code-block} diff
:caption: Folder Tree
    my_flutter_project/
    ├── android/
    ├── ios/
    ├── lib/
*   │   ├── main.dart
    │   └── ...
    ├── linux/
+   ├── native/
+   │   ├── hub/
+   │   │   ├── src/
+   │   │   └── Cargo.toml
+   │   └── README.md
    ├── web/
    ├── windows/
*   ├── .gitignore
+   ├── Cargo.toml
*   ├── pubspec.yaml
*   ├── README.md
    └── ...
```

Various comments are written in the actual code to help you understand the whole structure.

If you already have a Rust crate that you want to use here, just put it inside `./native` and set it as a dependency of the `hub` crate.

Now, by heading over to `./native/hub/src/lib.rs`, you can start writing Rust!

Example code for guidance can be found [here](https://github.com/cunarist/rinf/tree/main/flutter_package/example).
