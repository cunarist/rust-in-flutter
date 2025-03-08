use crate::{
    apply_rust_template, build_webassembly, check_internet_connection,
    generate_dart_code, load_verified_rinf_config,
    watch_and_generate_dart_code, RinfCommandError,
};
use clap::{Arg, ArgAction, ArgMatches, Command};
use owo_colors::OwoColorize;
use serde_yml::Value;
use std::env::current_dir;
use std::path::{Path, PathBuf};

// TODO: Remove string-based paths

pub fn run_command() -> Result<(), RinfCommandError> {
    // Check the internet connection status and remember it.
    let is_internet_connected = check_internet_connection();

    // Check if the ucrrent directory is Flutter app's root.
    let root_dir = current_dir().unwrap();
    if !is_flutter_app_project(&root_dir) {
        println!("{:?}", root_dir);
        Err(RinfCommandError::NotFlutterApp)?;
    }

    // Run a command from user input.
    let arg_matcher = create_arg_matcher();
    match arg_matcher.subcommand() {
        Some(("config", _)) => {
            let rinf_config = load_verified_rinf_config(&root_dir)?;
            println!("{}", rinf_config.dimmed());
        }
        Some(("template", _)) => {
            let rinf_config = load_verified_rinf_config(&root_dir)?;
            apply_rust_template(&root_dir, &rinf_config.message).unwrap();
        }
        Some(("gen", sub_m)) => {
            let watch = sub_m.get_flag("watch");
            let rinf_config = load_verified_rinf_config(&root_dir)?;
            if watch {
                watch_and_generate_dart_code(&root_dir, &rinf_config.message);
            } else {
                generate_dart_code(&root_dir, &rinf_config.message);
            }
        }
        Some(("wasm", sub_m)) => {
            let release = sub_m.get_flag("release");
            build_webassembly(&root_dir, release, is_internet_connected);
        }
        Some(("server", _)) => {
            // TODO: Dim the output
            let full_command = "flutter run \
                --web-header=Cross-Origin-Opener-Policy=same-origin \
                --web-header=Cross-Origin-Embedder-Policy=require-corp";
            println!("{}", full_command);
        }
        _ => unreachable!(), // TODO: Remove this unreachable
    }

    Ok(())
}

fn create_arg_matcher() -> ArgMatches {
    Command::new("rinf")
        .about("Helper commands for building apps using Rust in Flutter.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("config")
                .about("Show Rinf configuration resolved from `pubspec.yaml`."),
        )
        .subcommand(
            Command::new("template")
                .about("Apply Rust template to the current Flutter project."),
        )
        .subcommand(
            Command::new("gen")
                .about("Generate Dart code from Rust code with attributes.")
                .arg(
                    Arg::new("watch")
                        .short('w')
                        .long("watch")
                        .help("Continuously watches Rust files")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("wasm")
                .about("Build the WebAssembly module for the web.")
                .arg(
                    Arg::new("release")
                        .short('r')
                        .long("release")
                        .help("Builds in release mode")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("server")
                .about("Show how to run Flutter web server with web headers."),
        )
        .get_matches()
}

fn is_flutter_app_project(root_dir: &Path) -> bool {
    // Check if current folder is a Flutter app project.
    let spec_file = root_dir.join("pubspec.yaml");
    let publish_to = read_publish_to(&spec_file).unwrap();
    publish_to == "none"
}

fn read_publish_to(file_path: &PathBuf) -> Option<String> {
    let content = std::fs::read_to_string(file_path).unwrap();
    let yaml: Value = serde_yml::from_str(&content).unwrap();
    let field_value = yaml
        .as_mapping()
        .ok_or(RinfCommandError::Other)
        .unwrap()
        .get("publish_to")?;
    let config = field_value.as_str().unwrap().to_string();
    Some(config)
}
