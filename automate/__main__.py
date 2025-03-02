from os import chdir, remove, system
from pathlib import Path
from sys import argv

wrong_type: int = "HI"
from time import sleep

# TODO: Check types in code quality CI


ROOT_DIR = Path(__file__).parent.parent
COMMAND_ARG = 1


def replace_text_in_file(filepath: Path, change_from: str, change_to: str):
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content = content.replace(change_from, change_to)
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)


def update_cargokit():
    print("Updating CargoKit...")
    command = "git subtree pull"
    command += " --prefix flutter_package/cargokit"
    command += " https://github.com/irondash/cargokit.git"
    command += " main"
    system(command)


def prepare_test_app():
    filepath = ".gitignore"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content += "\n/test_app/"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

    command = "flutter create test_app"
    system(command)

    chdir("./test_app/")

    command = "dart pub add \"rinf:{'path':'../flutter_package'}\""
    system(command)
    command = "rinf template"
    while system(command) != 0:
        # Retry the command in case of failure,
        # possibly due to GitHub API rate limiting
        # associated with the 'protoc_prebuilt' crate.
        sleep(60)

    remove("Cargo.toml")

    # Enable the web target, since it's not enabled by default.
    replace_text_in_file(
        ROOT_DIR / "native" / "hub" / "src" / "lib.rs",
        "// use tokio_with_wasm::alias as tokio;",
        "use tokio_with_wasm::alias as tokio;",
    )
    replace_text_in_file(
        ROOT_DIR / "native" / "hub" / "Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_in_file(
        ROOT_DIR / "native" / "hub" / "Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )

    chdir("../")

    replace_text_in_file(
        ROOT_DIR / "Cargo.toml",
        "flutter_package/example/native/*",
        "test_app/native/*",
    )


def prepare_user_app():
    filepath = ".gitignore"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content += "\n/user_app/"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

    command = "flutter create user_app"
    system(command)

    chdir("./user_app/")

    command = "flutter pub add rinf"
    system(command)
    command = "rinf template"
    while system(command) != 0:
        # Retry the command in case of failure,
        # possibly due to GitHub API rate limiting
        # associated with the 'protoc_prebuilt' crate.
        sleep(60)

    remove("Cargo.toml")

    # Enable the web target, since it's not enabled by default.
    replace_text_in_file(
        ROOT_DIR / "native" / "hub" / "src" / "lib.rs",
        "// use tokio_with_wasm::alias as tokio;",
        "use tokio_with_wasm::alias as tokio;",
    )
    replace_text_in_file(
        ROOT_DIR / "native" / "hub" / "Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_in_file(
        ROOT_DIR / "native" / "hub" / "Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )

    chdir("../")

    replace_text_in_file(
        ROOT_DIR / "Cargo.toml",
        "flutter_package/example/native/*",
        "user_app/native/*",
    )
    replace_text_in_file(
        ROOT_DIR / "Cargo.toml",
        'rinf = { path = "./rust_crate" }',
        "",
    )


def prepare_example_app():
    chdir("./flutter_package/example")

    command = "rinf message"
    while system(command) != 0:
        # Retry the command in case of failure,
        # possibly due to GitHub API rate limiting
        # associated with the 'protoc_prebuilt' crate.
        sleep(60)


def run_command():
    if len(argv) < COMMAND_ARG + 1:
        print(
            "Automation option is not provided."
            "\nUse `python automate --help` to see all available operations."
        )
        return

    command = argv[COMMAND_ARG]
    match command:
        case "update-cargokit":
            update_cargokit()
        case "prepare-test-app":
            prepare_test_app()
        case "prepare-user-app":
            prepare_user_app()
        case "prepare-example-app":
            prepare_example_app()
        case _:
            print("No such option for automation is available.")


run_command()
