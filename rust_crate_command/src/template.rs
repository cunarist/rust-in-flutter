use crate::{generate_dart_code, run_dart_command, RinfConfigMessage};
use include_dir::{include_dir, Dir};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// TODO: Organize imports

static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/template");

/// Creates new folders and files in an existing Flutter project folder.
pub fn apply_rust_template(
    root_dir: &PathBuf,
    message_config: &RinfConfigMessage,
) -> io::Result<()> {
    // TODO: Use `message_config`

    // Copy basic folders needed for Rust to work
    dump_template(root_dir).unwrap();

    // Create workspace `Cargo.toml`
    write_cargo_toml(root_dir).unwrap();

    // Modify `.gitignore`
    update_gitignore(root_dir).unwrap();

    // Modify `README.md`
    update_readme(root_dir).unwrap();

    // Add Dart dependencies
    run_dart_command(&["pub", "add", "meta"]).unwrap();
    run_dart_command(&["pub", "add", "tuple"]).unwrap();

    // Modify `./lib/main.dart`
    update_main_dart(root_dir).unwrap();

    // Generate message code
    generate_dart_code(root_dir, message_config);

    println!("Rust template is now ready 🎉");
    Ok(())
}

/// Recursively extracts the embedded `TEMPLATE_DIR` to `dest_path`
fn dump_template(dest_path: &PathBuf) -> io::Result<()> {
    for entry in TEMPLATE_DIR.entries() {
        dump_entry(dest_path, entry)?;
    }
    Ok(())
}

fn dump_entry(
    dest_path: &PathBuf,
    entry: &include_dir::DirEntry,
) -> io::Result<()> {
    match entry {
        include_dir::DirEntry::Dir(dir) => {
            let dir_path = dest_path.join(dir.path());
            fs::create_dir_all(&dir_path)?;
            for sub_entry in dir.entries() {
                dump_entry(dest_path, sub_entry)?;
            }
        }
        include_dir::DirEntry::File(file) => {
            let file_path = dest_path.join(file.path());
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(file_path, file.contents())?;
        }
    }
    Ok(())
}

fn write_cargo_toml(root_dir: &Path) -> io::Result<()> {
    let cargo_toml_content = r#"
[workspace]
members = [\"./native/*\"]
resolver = \"2\"
"#;
    fs::write(root_dir.join("Cargo.toml"), cargo_toml_content)
}

fn update_gitignore(root_dir: &Path) -> io::Result<()> {
    let gitignore_path = root_dir.join(".gitignore");
    let mut content = fs::read_to_string(&gitignore_path).unwrap_or_default();
    if !content.contains("# Rust related") {
        content.push_str("\n# Rust related\n.cargo/\ntarget/\n");
    }
    if !content.contains("# Generated messages") {
        // TODO: Update the path
        content.push_str("\n# Generated messages\n*/**/messages/\n");
    }
    fs::write(gitignore_path, content)
}

fn update_readme(root_dir: &Path) -> io::Result<()> {
    let readme_path = root_dir.join("README.md");
    let mut content = fs::read_to_string(&readme_path).unwrap_or_default();
    let guide_section = "## Using Rust Inside Flutter";

    if !content.contains(guide_section) {
        content = content.trim_end().to_owned();
        content.push_str("\n\n");
        content.push_str(guide_section);
        content.push_str("\n\n");
        content.push_str(
r#"This project leverages Flutter for GUI and Rust for the backend logic,
utilizing the capabilities of the
[Rinf](https://pub.dev/packages/rinf) framework.

To run and build this app, you need to have
[Flutter SDK](https://docs.flutter.dev/get-started/install)
and [Rust toolchain](https://www.rust-lang.org/tools/install)
installed on your system.
You can check that your system is ready with the commands below.
Note that all the Flutter subcomponents should be installed.

```shell
rustc --version
flutter doctor
```

You also need to have the CLI tool for Rinf ready.

```shell
cargo install rinf
```

Signals sent between Dart and Rust are implemented using signal attributes.
If you've modified the signal structs, run the following command
to generate the corresponding Dart classes:

```shell
rinf gen
```

Now you can run and build this app just like any other Flutter projects.

```shell
flutter run
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rinf's [documentation](https://rinf.cunarist.com).
"#);
    }

    fs::write(readme_path, content)
}

fn update_main_dart(root_dir: &Path) -> io::Result<()> {
    let main_path = root_dir.join("lib/main.dart");
    if main_path.exists() {
        run_dart_command(&["format", "./lib/main.dart"])?;
        let mut content = fs::read_to_string(&main_path)?;
        // TODO: Update the import statement
        if !content.contains("messages/all.dart") {
            content = content.replacen(
                "import",
                "import 'package:rinf/rinf.dart';\
                \nimport 'generated.dart';\nimport",
                1,
            );
        }
        if !content.contains("initializeRust(assignRustSignal)") {
            content = content.replacen(
                "main() {",
                "main() async {\n    await initializeRust(assignRustSignal);",
                1,
            );
        }
        fs::write(main_path, content)?;
        run_dart_command(&["format", "./lib/main.dart"])?;
    }
    Ok(())
}
