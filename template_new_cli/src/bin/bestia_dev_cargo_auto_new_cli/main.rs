//! bestia_dev_cargo_auto_new_cli/src/bin/bestia_dev_cargo_auto_new_cli/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                print_greet_name(greet_name);
            }
            None => println!("Missing arguments `greet_name`."),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                // this can return an error. Here is the last place I can deal with the error.
                match upper_greet_name(greet_name) {
                    // do nothing
                    Ok(()) => (),
                    // log error from anyhow
                    Err(err) => log::error!("{}", err),
                }
            }
            None => println!("Missing arguments `greet_name`."),
        },
        _ => println!("Unrecognized arguments. Try `bestia_dev_cargo_auto_new_cli --help`"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
bestia_dev_cargo_auto_new_cli --help
bestia_dev_cargo_auto_new_cli print my_name
bestia_dev_cargo_auto_new_cli upper my_name
"#
    );
}

/// print my name
fn print_greet_name(greet_name: &str) {
    // call the function from the `lib.rs`
    println!("{}", bestia_dev_cargo_auto_new_cli::format_hello_phrase(greet_name));
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = bestia_dev_cargo_auto_new_cli::format_upper_hello_phrase(greet_name)?;
    println!("{}", upper);
    // return
    Ok(())
}