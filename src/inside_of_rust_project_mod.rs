// inside_of_rust_project_mod.rs

// region: use statements

// endregion

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";

pub fn parse_args(args: &mut std::env::Args) {
    // the first argument is the task: (no argument for help), new_auto, build, release,...
    // wooow! There is a difference if I call the standalone binary or as a cargo subcommand:
    // cargo-auto build     - build is the arg_1
    // cargo auto build     - build is the arg_2
    let arg_1 = args.next();
    match arg_1 {
        None => print_help_from_cargo_auto(),
        Some(task) => {
            if task != "auto" {
                // when calling as `cargo auto build`
                match_first_argument(&task, args);
            } else {
                // when calling as `cargo-auto build`
                let arg_2 = args.next();
                match arg_2 {
                    None => print_help_from_cargo_auto(),
                    Some(task) => match_first_argument(&task, args),
                }
            }
        }
    }
}

/// already exists automation_tasks_rs directory
fn already_exists_automation_tasks_rs() -> bool {
    // return
    crate::PATH_AUTOMATION_TASKS_RS.exists()
}

/// if there is no argument then print help  
/// if there exists `automation_tasks_rs/Cargo.toml` and `automation_tasks_rs/src/main.rs`  
/// call automation_tasks_rs with no arguments to print the help prepared in user defined automation_tasks_rs  
/// else print the help for `cargo auto new_auto`  
/// in development use: `cargo run`  
/// in runtime use: `cargo auto`  
fn print_help_from_cargo_auto() {
    if !crate::PATH_CARGO_TOML.exists() || !crate::PATH_SRC_MAIN_RS.exists() {
        println!(
            r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}
        
    To start using `cargo auto` inside your Rust project, you must create a new `automation_tasks_rs` directory with the command:
cargo auto new_auto

    ?? 2022 bestia.dev  MIT License github.com/bestia-dev/cargo-auto
"#
        );
    } else {
        build_automation_tasks_rs_if_needed();
        std::process::Command::new(crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

/// the first argument is the task: new_auto, build, release,...  
/// the task `new` is processed by `cargo-auto`,  
/// all other tasks are processed by the used defined `automation_tasks_rs`  
/// in development use: `cargo run -- new_auto`  
/// in development use: `cargo run -- build`  
/// in development use: `cargo run -- release`  
fn match_first_argument(task: &str, args: &mut std::env::Args) {
    if task == "completion" {
        completion();
    } else if task == "new_auto" {
        // this task is inside cargo-auto
        if already_exists_automation_tasks_rs() {
            println!( "{RED}Error: Directory automation_tasks_rs already exists. Cannot create new directory automation_tasks_rs.{RESET}" );
            // early exit
            std::process::exit(0);
        }
        new_auto();
    } else {
        // these tasks are user defined in automation_tasks_rs
        if !already_exists_automation_tasks_rs() {
            println!("{RED}Error: Directory automation_tasks_rs does not exist.{RESET}");
            print_help_from_cargo_auto();
            // early exit
            std::process::exit(0);
        }
        build_automation_tasks_rs_if_needed();
        // call automation_tasks_rs/target/debug/automation_tasks_rs with all the arguments
        let mut command = std::process::Command::new(crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str());
        command.arg(&task);
        while let Some(arg_x) = args.next() {
            command.arg(&arg_x);
        }
        let mut child = command.spawn().unwrap();
        child.wait().unwrap();
    }
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    /// println one, more or all sub_commands
    fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
        let mut sub_found = false;
        for sub_command in sub_commands.iter() {
            if sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
                sub_found = true;
            }
        }
        if sub_found == false {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{}", sub_command);
            }
        }
    }

    let args: Vec<String> = std::env::args().collect();
    let last_word = args[2].as_str();
    let mut word_being_completed = " ";
    if args.len() > 3 {
        word_being_completed = args[3].as_str();
    }
    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["new_auto"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
}

/// build if the date of Cargo.toml or main.rs is newer then of automation_tasks_rs/target/automation_tasks_rs
fn build_automation_tasks_rs_if_needed() {
    if !crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.exists() {
        build_project_automation_tasks_rs();
        // early return
        return ();
    }
    let modified_automation_tasks_rs = std::fs::metadata(crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str())
        .unwrap()
        .modified()
        .unwrap();
    let modified_cargo_toml = std::fs::metadata(crate::PATH_CARGO_TOML.as_os_str())
        .unwrap()
        .modified()
        .unwrap();
    let modified_main_rs = std::fs::metadata(crate::PATH_SRC_MAIN_RS.as_os_str())
        .unwrap()
        .modified()
        .unwrap();

    if modified_automation_tasks_rs < modified_cargo_toml || modified_automation_tasks_rs < modified_main_rs {
        build_project_automation_tasks_rs();
    }
}

/// build automation_tasks_rs
fn build_project_automation_tasks_rs() {
    // build in other directory (not in working current directory)
    // cargo build --manifest-path=dir/Cargo.toml
    std::process::Command::new("cargo")
        .arg("build")
        .arg("--manifest-path=automation_tasks_rs/Cargo.toml")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

/// copies the template to the `automation_tasks_rs` directory  
/// in development use: `cargo run -- new_auto`  
/// in runtime use: `cargo auto new_auto`  
fn new_auto() {
    crate::template_new_auto_mod::copy_to_files("automation_tasks_rs");
    build_automation_tasks_rs_if_needed();

    println!(
        r#"
    {YELLOW}`crate auto new_auto` generated the directory `automation_tasks_rs` in your main Rust project.
    You can open this new helper Rust project in a new Rust editor.
    View and edit the Rust code in `automation_tasks_rs`. It is independent from the main project.
    It will be automatically compiled on the next use of `crate auto task_name` command.
    The new directory will be added to your git commit.
    There is a local .gitignore file to avoid commit of the `target/` directory.
{RESET}"#
    );
    // call `cargo auto` to show the help of the new automation_tasks_rs
    std::process::Command::new("cargo")
        .arg("auto")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
