//! this strings are copied from the template_new_wasm folder
//! because when publishing to crates.io, only the main bin-executable is transferred

use crate::{GREEN, RED, RESET, YELLOW};

pub fn new_wasm(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Project name argument is missing: `cargo auto new_wasm project_name`{RESET}"),
        Some(project_name) => {
            copy_to_files(&project_name);
            println!("");
            println!("    {YELLOW}The command `crate auto new_wasm` generated the directory `{project_name}`{RESET}");
            println!("    {YELLOW}You can open this new Rust project `{project_name}` in a new Rust editor.{RESET}",);
            println!("    {YELLOW}For example VSCode:{RESET}");
            println!("{GREEN}code {project_name}{RESET}");
            println!("    {YELLOW}Then build with:{RESET}");
            println!("{GREEN}cargo auto build{RESET}");
            println!("    {YELLOW}and follow the detailed instructions.{RESET}");
        }
    }
}

pub fn copy_to_files(project_name: &str) {
    let folder_path = std::path::Path::new(project_name);
    std::fs::create_dir_all(folder_path).unwrap();
    for file_item in get_vec_file() {
        // rename/replace the project_name
        let file_name = file_item
            .file_name
            .replace("cargo_auto_template_new_wasm", project_name);
        let file_content = file_item
            .file_content
            .replace("cargo_auto_template_new_wasm", project_name);

        // create directory if needed
        std::fs::create_dir_all(folder_path.join(&file_name).parent().unwrap()).unwrap();
        std::fs::write(folder_path.join(&file_name), file_content.as_bytes()).unwrap();
    }
}

pub fn get_vec_file() -> Vec<crate::FileItem> {
    let mut vec_file = vec![];

    // region: files copied into strings by automation tasks
    vec_file.push(crate::FileItem{
            file_name :"README.md",
            file_content : r###"[//]: # (auto_md_to_doc_comments segment start A)

# cargo_auto_template_new_wasm

[//]: # (auto_cargo_toml_to_md start)

**template for a minimal WASM project for browser**  
***version: 2024.519.1012 date: 2024-05-19 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_auto_template_new_wasm)***  

[//]: # (auto_cargo_toml_to_md end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_template_new_wasm/blob/master/LICENSE)
 [![Rust](https://github.com/bestia-dev/cargo_auto_template_new_wasm/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_auto_template_new_wasm/)
 ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/638168303.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-262-green.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-30-blue.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-67-purple.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-19-orange.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)

[//]: # (auto_lines_of_code end)

Hashtags: #rustlang #tutorial #pwa #wasm #webassembly  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## template

Just like `cargo new` makes a soft and gentle introduction to Rust projects and development, I want to make the same for an in-browser WASM project with `cargo auto new_wasm`.  
Extremely simple, just the basic moving parts and use-cases.  
This simplest template does not have a PWA implementation or dedicated web server.

## Containerized Rust Development Environment (CRDE)

I recommend using the Containerized Rust Development Environment (CRDE) to write Rust projects. It contains wasm-pack and basic-http-server that this project needs.  
<https://github.com/bestia-dev/docker_rust_development>  

## HTML, CSS

The simple static HTML and CSS files are in `web_server_folder/cargo_auto_template_new_wasm`.  
Then the Rust code injects html elements into the DOM.  

## Web server and wasm

We will need the `basic-http-server` because browser security does not allow the loading of WASM modules from local files.  
Run the server in a separate VSCode terminal, so it can keep running all the time. In the first VSCode terminal, we can build the project and in the browser, we can refresh the page with F5.  

## Rust and wasm

Cargo.toml is very important to define the output as wasm library and the required dependencies to web-sys, js-sys, and wasm-bindgen.
Wasm starts from the src/lib.rs. On purpose, I added the main_mod.rs and lib_mod.rs to make the project structure similar to a Rust CLI project. The User Interface UI is completely different in-browser or CLI, but we can reuse the libraries that are UI agnostic.  It is smart to split a project that logic does not contain UI.

We use cargo auto for automation tasks. Run:
`cargo auto build`  
and follow the detailed instructions.

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/cargo_auto_template_new_wasm/index.html",
        file_content: r###"<!DOCTYPE html>
<html lang="en">

<head>
      <!-- classic header for a web page -->
      <title>cargo_auto_template_new_wasm</title>
      <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
      <meta name="Description" content="template for a minimal wasm project for browser">
      <meta name="author" content="https://github.com/bestia-dev/cargo_auto_template_new_wasm">    
      <link rel="stylesheet" href="css/basic_style.css">
      
</head>

<body>
      <!-- warning if javascript iis not enabled -->
      <noscript>
            <h2>
                  !!!???!!!<br>
                  This web app <br>
                  cannot work <br>
                  without javascript<br>
                  enabled<br>
                  !!!???!!!</h2>
      </noscript>
      <!-- display a text while waiting for wasm download. 
      This content will change from the wasm code.-->
      <div id="div_for_wasm_html_injecting">
            <h2>
                  Waiting to<br>
                  download <br>
                  the web app...<br>
                  This is <br>
                  very quick on fast<br>
                  networks...<br>
            </h2>
      </div>
      <p class="fc_red" id="div_for_errors"></p>
      <!-- import and init the wasm code -->
      <script type="module">
            import init from "./pkg/cargo_auto_template_new_wasm.js";
            init("./pkg/cargo_auto_template_new_wasm_bg.wasm");
      </script>
</body>

</html>"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/cargo_auto_template_new_wasm/css/basic_style.css",
        file_content: r###"html { 
    font-family: sans-serif;
    background-color: #000000;
    color: #FBF462;
}

h1{
    margin-left: auto;
    margin-right: auto;
    text-align: center;
}
p{
    margin-left: auto;
    margin-right: auto;
    text-align: center;
}
.small{
    font-size: 10px;
}

.input-wrap {
    position: relative;
	text-align: center;
  }

.button {
    display: inline-block;
    padding: 12px 18px;
    cursor: pointer;
    border-radius: 5px;
    background-color: #8ebf42;
    font-size: 16px;
    font-weight: bold;
    color: #fff;
  }

  .fc_red{
    color: red;
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".vscode/settings.json",
        file_content: r###"{
    "workbench.colorCustomizations": {
        "titleBar.activeForeground": "#fff",
        "titleBar.inactiveForeground": "#ffffffcc",
        "titleBar.activeBackground": "#477587",
        "titleBar.inactiveBackground": "#3F758DCC"
      },
      "spellright.language": [
        "en"
    ],
    "spellright.documentTypes": [
        "markdown",
        "latex",
        "plaintext"
    ],
    "rust-analyzer.showUnlinkedFileNotification": false,
    "cSpell.words": [
        "apos",
        "bestia",
        "bindgen",
        "cdylib",
        "CRDE",
        "endregion",
        "onchange",
        "onclick",
        "plantuml",
        "rustlang",
        "thiserror",
        "webassembly"
    ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "Cargo.toml",
        file_content: r###"[package]
name = "cargo_auto_template_new_wasm"
version = "2024.519.1012"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "template for a minimal wasm project for browser"
repository = "https://github.com/bestia-dev/cargo_auto_template_new_wasm"
readme = "README.md"
license = "MIT"
keywords = ["wasm"]
categories = ["learning"]
publish = false

[lib]
# cdylib is for the wasm module library
crate-type = ["cdylib"]

[dependencies]
unwrap = "1.2.1"
wasm-bindgen = "0.2.86"
console_error_panic_hook = "0.1.7"
js-sys = "0.3.63"
thiserror="1.0.40"
anyhow="1.0.71"
log = "0.4.17"
wasm-logger = "0.2.0"
wasm-rs-dbg = {version="0.1.2", default-features = false, features = ["console-log"]}

[dependencies.web-sys]
version = "0.3.63"
features = [
  "console",
  "Document",
  "Element",
  "HtmlElement",
  "HtmlInputElement",
  "Location",
  "Window",
]

[dev-dependencies]
wasm-bindgen-test = "0.3.36"

[profile.release]
panic = "abort"
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod.rs",
        file_content: r###"// src/lib_mod.rs
// This module is like a lib.rs module for a binary CLI executable.
// The `lib_mod.rs` must not contains any input/output interface stuff.
// So the program logic can be separate from the interface.

// The `main_mod.rs` contains all input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `lib_mod.rs` does not have any real code. All the code is in modules in separate files.
// The `lib_mod.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

use thiserror::Error;

// Instead of a hello_mod local module, we could use a UI agnostic crate library dependency.
// So the same library could be used for CLI and for WASM, that have vastly different UI.
mod hello_mod;

pub mod web_sys_mod;
pub use web_sys_mod as wsm;

pub use hello_mod::format_hello_phrase;
pub use hello_mod::format_upper_hello_phrase;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Name `{0}` is already uppercase.")]
    Uppercase(String),
    #[error("Unknown error.")]
    Unknown,
}

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
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod/web_sys_mod.rs",
        file_content: r###"//! src/web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! Trying to isolate/hide all javascript code and conversion here.

// region: use
// the macro unwrap! shows the TRUE location where the error has ocurred.
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
// use wasm_bindgen_futures::JsFuture;
use web_sys::console;
// use web_sys::{Request, RequestInit, Response};
// endregion: use

/// return the global window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// add event listener for button
pub fn add_listener_to_button(element_id: &str, fn_on_click_button: &'static (dyn Fn() + 'static)) {
    let handler_1 = Box::new(move || {
        fn_on_click_button();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    let html_element = get_html_element_by_id(element_id);
    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// set inner text
pub fn set_html_element_inner_text(element_id: &str, inner_text: &str) {
    let html_element = get_html_element_by_id(element_id);
    html_element.set_inner_text(inner_text);
}

/// WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
/// Only correctly html encoded strings can use this function.
/// set inner html into dom
pub fn set_html_element_inner_html(element_id: &str, inner_html: &str) {
    let html_element = get_element_by_id(element_id);
    html_element.set_inner_html(inner_html);
}

// open URL in new tab
pub fn open_url_in_new_tab(url: &str) {
    window().open_with_url_and_target(url, "_blank").unwrap();
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod/hello_mod.rs",
        file_content: r###"// src/hello_mod.rs

//! All the real code is inside modules in separate files (program logic).
//!
//! This doc-comments will be compiled into the `docs`.

/// format the hello phrase
pub fn format_hello_phrase(greet_name: &str) -> String {
    log::info!("start format_hello_phrase()");
    // return
    format!("Hello {}!", greet_name)
}

/// format the hello phrase with uppercase name
/// if it is already uppercase, return error with thiserror
pub fn format_upper_hello_phrase(greet_name: &str) -> Result<String, crate::LibraryError> {
    log::info!("start format_upper_hello_phrase()");
    // shadowing the same variable name:
    let upper_greet_name = make_uppercase(greet_name);
    if upper_greet_name == greet_name {
        return Err(crate::LibraryError::Uppercase(greet_name.to_string()));
    }

    // return
    Ok(format!("Hello {}!", &upper_greet_name))
}

/// return uppercase
pub fn make_uppercase(greet_name: &str) -> String {
    // return
    greet_name.to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_format_upper_hello_phrase() {
        assert_eq!(
            format_upper_hello_phrase("abcd").expect("error"),
            "Hello ABCD!"
        );
        assert!(format_upper_hello_phrase("ABCD").is_err());
    }

    #[test]
    pub fn test_make_uppercase() {
        assert_eq!(make_uppercase("abcd"), "ABCD");
        assert_eq!(make_uppercase("1234abcd"), "1234ABCD");
        assert_eq!(make_uppercase("čšž"), "ČŠŽ");
    }
}
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"src/main_mod.rs",
            file_content : r###"// src/main_mod.rs
// This module is like a main.rs module for a binary CLI executable.
// The `main_mod.rs` contains all input/output interface stuff.
// So the program logic can be separate from the interface.

// The `lib_mod.rs` must not contains any input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

use unwrap::unwrap;
use wasm_rs_dbg::dbg;

mod lib_mod;
pub use lib_mod::wsm;
pub use lib_mod::LibraryError;

/// entry point just like for cli-bin-executable
pub fn main() {
    // logging is essential for every project
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("main() started");

    // super simple argument parsing.
    // In browser we can use 'local routing' on url path with # fragment
    // http://localhost:4000/cargo_auto_template_new_wasm#arg_1/arg_2
    let location = wsm::window().location();
    let mut location_hash_fragment = unwrap!(location.hash());
    // dbg! is now writing to the console, crate wasm-rs-dbg
    dbg!(&location_hash_fragment);

    // in std::env::args() the nth(0) is the exe name. Let's make it similar.
    if !location_hash_fragment.is_empty() {
        // replace # with delimiter /
        location_hash_fragment.replace_range(..1, "/");
    }
    let location_hash_fragment = format!("cargo_auto_template_new_wasm{}", location_hash_fragment);
    dbg!(&location_hash_fragment);
    let args = location_hash_fragment.split("/");
    let args: Vec<&str> = args.collect();
    dbg!(&args);

    remove_downloading_message();

    // Since &str is Copy, you can avoid the creation of &&str by adding .copied()
    match args.get(1).copied() {
        None => page_with_inputs(),
        Some("help") => print_help(),
        Some("print") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => print_greet_name(greet_name),
                None => wsm::set_html_element_inner_text("div_for_errors","Error: Missing second argument for print."),
            }
        }
        Some("upper") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => {
                    // this can return an error. Here is the last place I can deal with the error.
                    match upper_greet_name(greet_name) {
                        // do nothing
                        Ok(()) => (),
                        // log error from anyhow
                        Err(err) => wsm::set_html_element_inner_text("div_for_errors",&format!("Error: {err}")),
                    }
                }
                None => wsm::set_html_element_inner_text("div_for_errors","Error: Missing second argument for upper."),
            }
        }
        _ => wsm::set_html_element_inner_text("div_for_errors","Error: Unrecognized arguments. Try \n http://localhost:4000/cargo_auto_template_new_wasm#help"),
    }
}


/// print help
fn print_help() {
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting",
        r#"
    Welcome to cargo_auto_template_new_wasm !
    
    This is a simple yet complete template for a WASM program written in Rust.
    The file structure is on purpose similar to a Rust CLI project and accepts similar arguments.

    http://localhost:4000/cargo_auto_template_new_wasm
    http://localhost:4000/cargo_auto_template_new_wasm#help
    http://localhost:4000/cargo_auto_template_new_wasm#print/world
    http://localhost:4000/cargo_auto_template_new_wasm#upper/world

    This command should return an error:
    http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD

    © 2024 bestia.dev  MIT License github.com/bestia-dev/cargo-auto
"#,
    );
}

/// render first page
pub fn page_with_inputs() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let html = r#"
<h1>Template_new_wasm</h1>
<p>Write a command in the Argument 1: print or upper</p>
<div class="input-wrap">
    <label for="arg_1">Argument 1:</label>  
    <input style="width:20%;" type="text" id="arg_1" value="upper"/>
</div>
<p>Write a name in the Argument 2: world or WORLD</p>
<div class="input-wrap">
    <label for="arg_2">Argument 2:</label>  
    <input style="width:20%;" type="text" id="arg_2" value="world"/>
</div>
<p>Click on Run</p>
<div class="input-wrap">
    <input type="button" class="button" id="btn_run" value="Run"/>
</div>
<p class="small">bestia.dev</p>
        "#;

    // WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
    // Only correctly html encoded strings can use this function.
    wsm::set_html_element_inner_html("div_for_wasm_html_injecting",html);
    wsm::add_listener_to_button("btn_run", &on_click_btn_run);
}

/// the listener calls this function
fn on_click_btn_run() {
    let arg_1 = wsm::get_input_element_value_string_by_id("arg_1");
    let arg_2 = wsm::get_input_element_value_string_by_id("arg_2");
    if !arg_1.is_empty() && !arg_2.is_empty() {
        // pass arguments as URL in a new tab
        let url = format!("/cargo_auto_template_new_wasm#{arg_1}/{arg_2}");
        wsm::open_url_in_new_tab(&url);
    } else {
        // write on the same web page
        wsm::set_html_element_inner_text(
            "div_for_errors",
            &format!("Error: Both arguments are mandatory."),
        );
    }
}

// remove downloading message
fn remove_downloading_message() {
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting","");
}

/// print my name
fn print_greet_name(greet_name: &str) {
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting",&format!(
r#"The result is
{}
"#,
    lib_mod::format_hello_phrase(greet_name)
    ));
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = lib_mod::format_upper_hello_phrase(greet_name)?;
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting",&format!(
r#"The result is
{upper}
"#
    ));
    // return
    Ok(())
}
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "src/lib.rs",
        file_content: r###"//! src/lib.rs
//! This file has just the wasm_bindgen_start() function
//! and calls into main_mod.rs.
//! So the structure of the project modules can be similar to a binary CLI executable.

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo-auto  
//!
//! **cargo-auto - automation tasks written in Rust language for the build process of Rust projects**  
//! ***version: 2024.209.504 date: 2024-02-18 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo-auto)***  
//!
//!  ![logo](https://raw.githubusercontent.com/bestia-dev/cargo-auto/main/images/logo/logo_cargo_auto.svg)
//!  ![status](https://img.shields.io/badge/maintained-green)
//!  ![status](https://img.shields.io/badge/ready_for_use-green)
//!
//!  [![crates.io](https://img.shields.io/crates/v/cargo-auto.svg)](https://crates.io/crates/cargo-auto)
//!  [![Documentation](https://docs.rs/cargo-auto/badge.svg)](https://docs.rs/cargo-auto/)
//!  [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo-auto.svg)](https://web.crev.dev/rust-reviews/crate/cargo-auto/)
//!  [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo-auto/)
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo-auto/blob/master/LICENSE)
//!  [![Rust](https://github.com/bestia-dev/cargo-auto/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/cargo-auto/)
//!  ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/959103982.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-9191-green.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-1227-blue.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-430-purple.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-6332-orange.svg)](https://github.com/bestia-dev/cargo-auto/)
//!
//! Hashtags: #rustlang #tutorial #buildtool #developmenttool #cli  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//!
//! ## Try it
//!
//! First, we will use `cargo-auto` to create a new empty CLI Rust project similar to `cargo new`, but with a more complete project structure.  
//!
//!  ```bash
//! cargo install cargo-auto
//! cargo auto new_cli my_hello_project
//! cd my_hello_project
//! cargo auto
//! # it lists all the prepared automation tasks
//! # try a few
//! cargo auto build
//! cargo auto release
//! cargo auto doc
//! cargo auto test
//! ```
//!
//! We can also add `automation tasks` to an existing Rust project.
//! Inside your Rust project directory (the one with Cargo.toml) run:  
//!
//! ```bash
//! cargo auto new_auto
//! cargo auto
//! # it lists all the prepared automation tasks
//! # try to build
//! cargo auto build
//! ```
//!
//! Congratulations! You are already using `cargo-auto`. Simple as that.  
//! Now you can modify the tasks to your needs. It is all Rust language.  
//!
//! ## Motivation
//!
//! Cargo is a great tool for building Rust projects. It has all the basics: `cargo build`, `cargo build --release`, `cargo fmt`, `cargo test`, `cargo doc`,...  
//! But sometimes we need to do more things like copying some files, publishing to FTP, or entering long commands. These repetitive tasks must be automated.  
//! Task automation makes work easier and faster, and simplifies the workflow while improving the consistency and accuracy of workflows.  
//! This is also sometimes referred to as "workflow automation."  
//! There are many different build systems and task runners there: `make`, `cmake`, `shell scripts`, `cargo-xtask`, `cargo-make`, `cargo-task`, `cargo-script`, `cargo-run-script`, `runner`, `python scripts`, `powershell scripts`, `cmd prompt scripts`, ...  
//! Sadly there is no standard in the Rust community for now.  
//! I want something similar to [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html), so I can write my "tasks" in pure Rust I don't want to learn another meta language with weird syntax and difficulty to debug. So I will make something really simple, easy, rusty, and extensible.  
//!
//! ## cargo auto subcommand
//!
//! The command `cargo install cargo-auto` will add a new subcommand to cargo:
//!
//! ```bash
//! cargo auto
//! ```
//!
//! This binary is super simple. It has only 1 trivial dependency: `lazy_static`.  
//! The binary only reads the CLI arguments and runs the `automation_tasks_rs` binary with them. If needed it will compile `automation_tasks_rs` first.  
//! The code-flow of the source code of `cargo-auto` is simple, fully commented, and straightforward to audit.  
//! The source code is on [GitHub](https://github.com/bestia-dev/cargo-auto) with MIT open-source licensing.  
//!
//! ## bash auto-completion
//!
//! With the help of the crate [dev_bestia_cargo_completion](https://crates.io/crates/dev_bestia_cargo_completion), the commands `cargo` and `cargo auto` get bash auto-completion. Try it!  
//!
//! ## cargo auto new_cli
//!
//! I like very much that Rust has the command `cargo new project_name`. It creates a super simple Rust Hello project that can be built and run immediately. But this example is too simple. It lacks the basic file structures of a serious CLI program.  
//! I composed an opinionated template for a Rust CLI project. It is easy to run:
//!
//! ```bash
//! cargo auto new_cli project_name
//! ```
//!
//! ## cargo auto new_wasm
//!
//! I composed an opinionated template for a simple Rust WASM project for a browser. It is very similar to the new_cli template but for WASM.  
//! It is easy to run:
//!
//! ```bash
//! cargo auto new_wasm project_name
//! # then
//! cd project_name
//! cargo auto build
//! # follow detailed instructions
//! ```
//!
//! ## cargo auto new_pwa_wasm
//!
//! I composed an opinionated template for a simple Rust PWA-WASM project for a browser. It is very similar to the new_cli template but for WASM. It adds the PWA standard functionality to work as an offline app.  
//! The template needs the title, name, long name, and description inside a `pwa.json5` file and the `icon512x512.png` file for the icons.  
//! It is easy to run:
//!
//! ```bash
//! cargo auto new_pwa_wasm
//! # on first run it will just create the `pwa.json5` and `icon512x512.png` files
//! # modify these files for your new app
//! cargo auto new_pwa_wasm
//! # then
//! cd project_name
//! cargo auto build
//! # follow detailed instructions
//! ```
//!
//! ## scripting with rust
//!
//! Rust is a compiled language. It is not really a scripting or interpreted language. But the compilation of small projects is really fast and can be ignored. Subsequent calls will use the already-built binary so the speed will be even faster.  
//! This tool `cargo-auto` is meant for Rust projects, so it means that all the Rust infrastructure is already in place.  
//!
//! ## automation_tasks_rs helper project
//!
//! The command `cargo auto new_auto` will create a new directory `automation_tasks_rs` with a template for a helper Rust project in the root directory of your `main Rust project`. It should not interfere with the main Rust project. This directory will be added to git commits and pushed to remote repositories as part of the main project. It has its own `.gitignore` to avoid committing to its target directory.  
//! The `automation_tasks_rs` helper project contains user-defined tasks in Rust code. Your tasks. This helper project should be opened in a new editor starting from the `automation_tasks_rs` directory. It does not share dependencies with the main project. It is completely separate and independent.  
//! You can edit it and add your dependencies and Rust code. No limits. Freedom of expression.  
//! This is now your code, your tasks, and your helper Rust project!  
//! Because only you know what you want to automate and how to do it.  
//! Never write secrets, passwords, passcodes, or tokens inside your Rust code. Because then it is pushed to GitHub and the whole world can read it in the next second!
//! Basic example (most of the useful functions are already there):  
//!
//! ```rust
//! /// match arguments and call tasks functions
//! fn match_arguments_and_call_tasks(mut args: std::env::Args){
//!     // the first argument is the user defined task: (no argument for help), build, release,...
//!     let arg_1 = args.next();
//!     match arg_1 {
//!         None => print_help(),
//!         Some(task) => {            
//!             println!("Running auto task: {}", &task);
//!             if &task == "build"{
//!                 task_build();
//!             } else if &task == "release" {
//!                 task_release();
//!             } else if &task == "doc" {
//!                 task_doc();
//!             } else {
//!                 println!("Task {} is unknown.", &task);
//!                 print_help();
//!             }
//!         }
//!     }
//! }
//!
//! /// write a comprehensible help for user defined tasks
//! fn print_help() {
//!     println!(r#"
//!     User defined tasks in automation_tasks_rs:
//! cargo auto build - builds the crate in debug mode
//! cargo auto release - builds the crate in release mode
//! cargo auto docs - builds the docs
//! "#);
//! }
//!
//! // region: tasks
//!
//! /// cargo build
//! fn task_build() {
//!     run_shell_command("cargo fmt");
//!     run_shell_command("cargo build");
//! }
//!
//! /// cargo build --release
//! fn task_release() {
//!     run_shell_command("cargo fmt");
//!     run_shell_command("cargo build --release");
//! }
//!
//! /// cargo doc, then copies to /docs/ folder, because this is a github standard folder
//! fn task_doc() {
//!     run_shell_command("cargo doc --no-deps --document-private-items");
//!     // copy target/doc into docs/ because it is github standard
//!     run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
//!     // Create simple index.html file in docs directory
//!     run_shell_command(&format!(
//!         "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
//!         cargo_toml.package_name().replace("-","_")
//!     ));
//!     run_shell_command("cargo fmt");
//! }
//!
//! // endregion: tasks
//!
//! ```
//!
//! ## more complex tasks
//!
//! You can write more complex tasks in Rust language.  
//! For example in this project I use automation to create GitHub Releases: <https://github.com/bestia-dev/dropbox_backup_to_external_disk>  
//! Here is a pretty complex workspace with more sub-projects:  
//! <https://github.com/bestia-dev/cargo_crev_reviews_workspace>  
//! There is no end to your imagination. If you write something that looks like it can help other developers, please share it with me and I will add it here.
//!
//! ## Development details
//!
//! Read the development details in a separate md file:  
//! [DEVELOPMENT.md](https://github.com/bestia-dev/cargo-auto/blob/main/DEVELOPMENT.md)
//!
//! ## Changelog
//!
//! Read the changelog in a separate md file:  
//! [RELEASES.md](https://github.com/bestia-dev/cargo-auto/blob/main/RELEASES.md)
//!
//! ## TODO
//!
//! new wasm, new wasm_pwa, new wasm_pwa_server, new wasm_pwa_server_pgrsql
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

use wasm_bindgen::prelude::*;

mod main_mod;
pub use main_mod::wsm;
pub use main_mod::LibraryError;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    wsm::debug_write(&format!(
        "cargo_auto_template_new_wasm v{}",
        env!("CARGO_PKG_VERSION")
    ));

    main_mod::main();
    // return
    Ok(())
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitattributes",
        file_content: r###"# Specific git config for the project

# Declare files that will always have LF line endings on checkout.
*.rs text eol=lf
*.toml text eol=lf
*.md text eol=lf
*.json text eol=lf
*.json5 text eol=lf
*.lock text eol=lf
*.yml text eol=lf
*.html text eol=lf
*.js text eol=lf
*.css text eol=lf
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitignore",
        file_content: r###"# Generated by Cargo
# will have compiled files and executables
/target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
# Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# result of compilation does not need to go to repository
/pkg/

# not needed in commits, but also not a problem if they are committed
/.automation_tasks_rs_file_hashes.json
/.auto_version_from_date.json
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.vscode/settings.json",
        file_content: r###"{
    "cSpell.words": [
        "alloc",
        "bestia",
        "endregion",
        "new_cli",
        "octocrab",
        "plantuml",
        "zcvf"
    ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.toml",
        file_content: r###"
[package]
name = "automation_tasks_rs"
version = "1.0.1"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "cargo auto - automation tasks written in Rust language"
publish = false

[dependencies]
cargo_auto_lib = "1.1.2""###,
    });
    vec_file.push(crate::FileItem{
            file_name :"automation_tasks_rs/src/main.rs",
            file_content : r###"//! automation_tasks_rs for cargo_auto_template_new_wasm

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;
// region: library with basic automation tasks

// use cargo_auto_github_lib::*;

fn main() {
    cl::exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !{RESET}
    {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

    {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET}{YELLOW} - builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET}{YELLOW} - builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET}{YELLOW} - builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET}{YELLOW} - runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}{YELLOW} - commits with message and push with mandatory message{RESET}
    {YELLOW}(If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.){RESET}
{GREEN}cargo auto publish_to_web - publish to web, git tag{RESET}
    {YELLOW}(You need credentials for publishing over SSH. Use ssh-agent and ssh-add to store the credentials for SSH.){RESET}

    {YELLOW}© 2024 bestia.dev  MIT License github.com/bestia-dev/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd() {
    /*
        println!(r#"{YELLOW}run examples:{RESET}
    {GREEN}cargo run --example example1{RESET}
    "#);
    */
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_web"];
        cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// wasm-pack build
fn task_build() {
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/cargo_auto_template_new_wasm/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server
    in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#print/world{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto release{RESET}
"#
    );
}

/// wasm-pack build --release
fn task_release() {
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/cargo_auto_template_new_wasm/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server
    in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm{RESET}    
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#print/world{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto doc{RESET}
"#
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
    cl::auto_md_to_doc_comments();

    cl::run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    cl::run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    cl::run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-", "_")
    ));

    // region: tidy HTML
    // The HTML generated by `cargo doc` is ugly and difficult to `git diff`
    // tidy HTML is a HTML checker and formatter installed on most Linuxes.
    // If it is not installed run: `sudo apt install -y tidy`
    // From the bash you can install it inside the podman container like this:
    // `podman exec --user root rust_dev_vscode_cnt apt install -y tidy`
    //
    // First we check if tidy is installed on the system
    // Run a dummy command and write the std/err output to tidy_warnings.txt.
    // The command `2>` will overwrite the file and not append like `2>>`.
    cl::run_shell_command("tidy xxx 2> docs/tidy_warnings.txt");
    // Check if it contains `command not found`
    let text = std::fs::read_to_string("docs/tidy_warnings.txt").unwrap();
    // don't need this file anymore
    cl::run_shell_command("rm -f docs/tidy_warnings.txt");
    if !text.contains("command not found") {
        // Use tidy HTML to format the docs/*.html files to be human readable and usable for git diff.
        // Options: -m modify file, -q quiet suppress nonessential output, -w wrap at 160, -i indent 2 spaces
        // The warnings and errors are appended to the file docs/tidy_warnings.txt
        cl::run_shell_command(
            r#"find ./docs -name '*.html' -type f -print -exec tidy -mq -w 160 -i 2 '{}' \; >> docs/tidy_warnings.txt 2>&1 "#,
        );
    }
    // endregion: tidy HTML
    cl::run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
    {YELLOW}{RESET}"#
    );
}

/// cargo test
fn task_test() {
    cl::run_shell_command("cargo test");
    println!(
        r#"
    {YELLOW}After `cargo auto test`. If ok, then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Message for commit is mandatory.{RESET}"),
        Some(message) => {
            // separate commit for docs if they changed, to not make a lot of noise in the real commit
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
            // the real commit of code
            cl::run_shell_command(&format!(
                r#"git add -A && git diff --staged --quiet || git commit -m "{}" "#,
                message
            ));
            cl::run_shell_command("git push");
            println!(
                r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
            );
        }
    }
}

/// publish to web
fn task_publish_to_web() {
    println!(r#"{YELLOW}Use ssh-agent and ssh-add to store your credentials for publish to web.{RESET}"#);
    let cargo_toml = cl::CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    cl::run_shell_command(&shell_command);
    let shell_command = format!(
        "rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/{package_name}/web_server_folder/ project_author@project_homepage:/var/www/project_homepage/pwa_short_name/",
        package_name = cargo_toml.package_name()
    );
    cl::run_shell_command(&shell_command);
    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_web`, 
    check 
https://bestia.dev/{package_name}
{RESET}"#,
        package_name = cargo_toml.package_name()
    );
}

// endregion: tasks
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.gitignore",
        file_content: r###"/target

# not needed in commits, but also not a problem if they are committed
/.file_hashes.json
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.lock",
        file_content: r###"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "addr2line"
version = "0.21.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8a30b2e23b9e17a9f90641c7ab1549cd9b44f296d3ccbf309d2863cfe398a0cb"
dependencies = [
 "gimli",
]

[[package]]
name = "adler"
version = "1.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f26201604c87b1e01bd3d98f8d5d9a8fcbb815e8cedb41ffccbeb4bf593a35fe"

[[package]]
name = "adler32"
version = "1.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "aae1277d39aeec15cb388266ecc24b11c80469deae6067e17a1a7aa9e5c1f234"

[[package]]
name = "aho-corasick"
version = "1.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b2969dcb958b36655471fc61f7e416fa76033bdd4bfed0678d8fee1e2d07a1f0"
dependencies = [
 "memchr",
]

[[package]]
name = "android-tzdata"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e999941b234f3131b00bc13c22d06e8c5ff726d1b6318ac7eb276997bbb4fef0"

[[package]]
name = "android_system_properties"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "819e7219dbd41043ac279b19830f2efc897156490d7fd6ea916720117ee66311"
dependencies = [
 "libc",
]

[[package]]
name = "anyhow"
version = "1.0.79"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "080e9890a082662b09c1ad45f567faeeb47f22b5fb23895fbe1e651e718e25ca"

[[package]]
name = "arrayref"
version = "0.3.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6b4930d2cb77ce62f89ee5d5289b4ac049559b1c45539271f5ed4fdc7db34545"

[[package]]
name = "autocfg"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"

[[package]]
name = "automation_tasks_rs"
version = "1.0.1"
dependencies = [
 "cargo_auto_lib",
]

[[package]]
name = "backtrace"
version = "0.3.69"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2089b7e3f35b9dd2d0ed921ead4f6d318c27680d4a5bd167b3ee120edb105837"
dependencies = [
 "addr2line",
 "cc",
 "cfg-if 1.0.0",
 "libc",
 "miniz_oxide",
 "object",
 "rustc-demangle",
]

[[package]]
name = "base64"
version = "0.21.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9d297deb1925b89f2ccc13d7635fa0714f12c87adce1c75356b39ca9b7178567"

[[package]]
name = "base64ct"
version = "1.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8c3c1a368f70d6cf7302d78f8f7093da241fb8e8807c05cc9e51a125895a6d5b"

[[package]]
name = "bitflags"
version = "1.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a"

[[package]]
name = "bitflags"
version = "2.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ed570934406eb16438a4e976b1b4500774099c13b8cb96eec99f620f05090ddf"

[[package]]
name = "block-buffer"
version = "0.10.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3078c7629b62d3f0439517fa394996acacc5cbc91c5a20d8c658e77abd503a71"
dependencies = [
 "generic-array",
]

[[package]]
name = "bumpalo"
version = "3.14.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7f30e7476521f6f8af1a1c4c0b8cc94f0bee37d91763d0ca2665f299b6cd8aec"

[[package]]
name = "bytes"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a2bd12c1caf447e69cd4528f47f94d203fd2582878ecb9e9465484c4148a8223"

[[package]]
name = "cargo_auto_lib"
version = "1.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e1f87d809127bc54c9dfe61007640dd082f38d6978c344b792bdf021152c9108"
dependencies = [
 "anyhow",
 "base64ct",
 "cargo_toml",
 "chrono",
 "data-encoding",
 "deflate",
 "filetime",
 "glob",
 "lazy_static",
 "radix64",
 "reader_for_microxml",
 "regex",
 "reqwest",
 "ring",
 "semver",
 "serde",
 "serde_derive",
 "serde_json",
 "sha2",
 "termion",
 "thiserror",
 "toml",
 "unwrap",
]

[[package]]
name = "cargo_toml"
version = "0.19.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3dc9f7a067415ab5058020f04c60ec7b557084dbec0e021217bbabc7a8d38d14"
dependencies = [
 "serde",
 "toml",
]

[[package]]
name = "cc"
version = "1.0.83"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f1174fb0b6ec23863f8b971027804a42614e347eafb0a95bf0b12cdae21fc4d0"
dependencies = [
 "libc",
]

[[package]]
name = "cfg-if"
version = "0.1.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822"

[[package]]
name = "cfg-if"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"

[[package]]
name = "chrono"
version = "0.4.33"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9f13690e35a5e4ace198e7beea2895d29f3a9cc55015fcebe6336bd2010af9eb"
dependencies = [
 "android-tzdata",
 "iana-time-zone",
 "js-sys",
 "num-traits",
 "wasm-bindgen",
 "windows-targets 0.52.0",
]

[[package]]
name = "core-foundation"
version = "0.9.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "91e195e091a93c46f7102ec7818a2aa394e1e1771c3ab4825963fa03e45afb8f"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "core-foundation-sys"
version = "0.8.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "06ea2b9bc92be3c2baa9334a323ebca2d6f074ff852cd1d7b11064035cd3868f"

[[package]]
name = "cpufeatures"
version = "0.2.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "53fe5e26ff1b7aef8bca9c6080520cfb8d9333c7568e1829cef191a9723e5504"
dependencies = [
 "libc",
]

[[package]]
name = "crypto-common"
version = "0.1.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1bfb12502f3fc46cca1bb51ac28df9d618d813cdc3d2f25b9fe775a34af26bb3"
dependencies = [
 "generic-array",
 "typenum",
]

[[package]]
name = "data-encoding"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7e962a19be5cfc3f3bf6dd8f61eb50107f356ad6270fbb3ed41476571db78be5"

[[package]]
name = "deflate"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c86f7e25f518f4b81808a2cf1c50996a61f5c2eb394b2393bd87f2a4780a432f"
dependencies = [
 "adler32",
]

[[package]]
name = "digest"
version = "0.10.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9ed9a281f7bc9b7576e61468ba615a66a5c8cfdff42420a70aa82701a3b1e292"
dependencies = [
 "block-buffer",
 "crypto-common",
]

[[package]]
name = "encoding_rs"
version = "0.8.33"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7268b386296a025e474d5140678f75d6de9493ae55a5d709eeb9dd08149945e1"
dependencies = [
 "cfg-if 1.0.0",
]

[[package]]
name = "equivalent"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5443807d6dff69373d433ab9ef5378ad8df50ca6298caf15de6e52e24aaf54d5"

[[package]]
name = "errno"
version = "0.3.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a258e46cdc063eb8519c00b9fc845fc47bcfca4130e2f08e88665ceda8474245"
dependencies = [
 "libc",
 "windows-sys 0.52.0",
]

[[package]]
name = "fastrand"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "25cbce373ec4653f1a01a31e8a5e5ec0c622dc27ff9c4e6606eefef5cbbed4a5"

[[package]]
name = "filetime"
version = "0.2.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1ee447700ac8aa0b2f2bd7bc4462ad686ba06baa6727ac149a2d6277f0d240fd"
dependencies = [
 "cfg-if 1.0.0",
 "libc",
 "redox_syscall",
 "windows-sys 0.52.0",
]

[[package]]
name = "fnv"
version = "1.0.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1"

[[package]]
name = "foreign-types"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f6f339eb8adc052cd2ca78910fda869aefa38d22d5cb648e6485e4d3fc06f3b1"
dependencies = [
 "foreign-types-shared",
]

[[package]]
name = "foreign-types-shared"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "00b0228411908ca8685dba7fc2cdd70ec9990a6e753e89b6ac91a84c40fbaf4b"

[[package]]
name = "form_urlencoded"
version = "1.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e13624c2627564efccf4934284bdd98cbaa14e79b0b5a141218e507b3a823456"
dependencies = [
 "percent-encoding",
]

[[package]]
name = "futures-channel"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eac8f7d7865dcb88bd4373ab671c8cf4508703796caa2b1985a9ca867b3fcb78"
dependencies = [
 "futures-core",
]

[[package]]
name = "futures-core"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dfc6580bb841c5a68e9ef15c77ccc837b40a7504914d52e47b8b0e9bbda25a1d"

[[package]]
name = "futures-io"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a44623e20b9681a318efdd71c299b6b222ed6f231972bfe2f224ebad6311f0c1"

[[package]]
name = "futures-sink"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9fb8e00e87438d937621c1c6269e53f536c14d3fbd6a042bb24879e57d474fb5"

[[package]]
name = "futures-task"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "38d84fa142264698cdce1a9f9172cf383a0c82de1bddcf3092901442c4097004"

[[package]]
name = "futures-util"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3d6401deb83407ab3da39eba7e33987a73c3df0c82b4bb5813ee871c19c41d48"
dependencies = [
 "futures-core",
 "futures-io",
 "futures-task",
 "memchr",
 "pin-project-lite",
 "pin-utils",
 "slab",
]

[[package]]
name = "generic-array"
version = "0.14.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85649ca51fd72272d7821adaf274ad91c288277713d9c18820d8499a7ff69e9a"
dependencies = [
 "typenum",
 "version_check",
]

[[package]]
name = "getrandom"
version = "0.2.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "190092ea657667030ac6a35e305e62fc4dd69fd98ac98631e5d3a2b1575a12b5"
dependencies = [
 "cfg-if 1.0.0",
 "libc",
 "wasi",
]

[[package]]
name = "gimli"
version = "0.28.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4271d37baee1b8c7e4b708028c57d816cf9d2434acb33a549475f78c181f6253"

[[package]]
name = "glob"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d2fabcfbdc87f4758337ca535fb41a6d701b65693ce38287d856d1674551ec9b"

[[package]]
name = "h2"
version = "0.3.24"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bb2c4422095b67ee78da96fbb51a4cc413b3b25883c7717ff7ca1ab31022c9c9"
dependencies = [
 "bytes",
 "fnv",
 "futures-core",
 "futures-sink",
 "futures-util",
 "http",
 "indexmap",
 "slab",
 "tokio",
 "tokio-util",
 "tracing",
]

[[package]]
name = "hashbrown"
version = "0.14.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "290f1a1d9242c78d09ce40a5e87e7554ee637af1351968159f4952f028f75604"

[[package]]
name = "hermit-abi"
version = "0.3.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d0c62115964e08cb8039170eb33c1d0e2388a256930279edca206fff675f82c3"

[[package]]
name = "http"
version = "0.2.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8947b1a6fad4393052c7ba1f4cd97bed3e953a95c79c92ad9b051a04611d9fbb"
dependencies = [
 "bytes",
 "fnv",
 "itoa",
]

[[package]]
name = "http-body"
version = "0.4.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7ceab25649e9960c0311ea418d17bee82c0dcec1bd053b5f9a66e265a693bed2"
dependencies = [
 "bytes",
 "http",
 "pin-project-lite",
]

[[package]]
name = "httparse"
version = "1.8.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d897f394bad6a705d5f4104762e116a75639e470d80901eed05a860a95cb1904"

[[package]]
name = "httpdate"
version = "1.0.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "df3b46402a9d5adb4c86a0cf463f42e19994e3ee891101b1841f30a545cb49a9"

[[package]]
name = "hyper"
version = "0.14.28"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bf96e135eb83a2a8ddf766e426a841d8ddd7449d5f00d34ea02b41d2f19eef80"
dependencies = [
 "bytes",
 "futures-channel",
 "futures-core",
 "futures-util",
 "h2",
 "http",
 "http-body",
 "httparse",
 "httpdate",
 "itoa",
 "pin-project-lite",
 "socket2",
 "tokio",
 "tower-service",
 "tracing",
 "want",
]

[[package]]
name = "hyper-tls"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d6183ddfa99b85da61a140bea0efc93fdf56ceaa041b37d553518030827f9905"
dependencies = [
 "bytes",
 "hyper",
 "native-tls",
 "tokio",
 "tokio-native-tls",
]

[[package]]
name = "iana-time-zone"
version = "0.1.60"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e7ffbb5a1b541ea2561f8c41c087286cc091e21e556a4f09a8f6cbf17b69b141"
dependencies = [
 "android_system_properties",
 "core-foundation-sys",
 "iana-time-zone-haiku",
 "js-sys",
 "wasm-bindgen",
 "windows-core",
]

[[package]]
name = "iana-time-zone-haiku"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f31827a206f56af32e590ba56d5d2d085f558508192593743f16b2306495269f"
dependencies = [
 "cc",
]

[[package]]
name = "idna"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "634d9b1461af396cad843f47fdba5597a4f9e6ddd4bfb6ff5d85028c25cb12f6"
dependencies = [
 "unicode-bidi",
 "unicode-normalization",
]

[[package]]
name = "indexmap"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "824b2ae422412366ba479e8111fd301f7b5faece8149317bb81925979a53f520"
dependencies = [
 "equivalent",
 "hashbrown",
]

[[package]]
name = "ipnet"
version = "2.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f518f335dce6725a761382244631d86cf0ccb2863413590b31338feb467f9c3"

[[package]]
name = "itoa"
version = "1.0.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b1a46d1a171d865aa5f83f92695765caa047a9b4cbae2cbf37dbd613a793fd4c"

[[package]]
name = "js-sys"
version = "0.3.68"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "406cda4b368d531c842222cf9d2600a9a4acce8d29423695379c6868a143a9ee"
dependencies = [
 "wasm-bindgen",
]

[[package]]
name = "lazy_static"
version = "1.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646"

[[package]]
name = "libc"
version = "0.2.153"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9c198f91728a82281a64e1f4f9eeb25d82cb32a5de251c6bd1b5154d63a8e7bd"

[[package]]
name = "libredox"
version = "0.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3af92c55d7d839293953fcd0fda5ecfe93297cfde6ffbdec13b41d99c0ba6607"
dependencies = [
 "bitflags 2.4.2",
 "libc",
 "redox_syscall",
]

[[package]]
name = "linux-raw-sys"
version = "0.4.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "01cda141df6706de531b6c46c3a33ecca755538219bd484262fa09410c13539c"

[[package]]
name = "log"
version = "0.4.20"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b5e6163cb8c49088c2c36f57875e58ccd8c87c7427f7fbd50ea6710b2f3f2e8f"

[[package]]
name = "memchr"
version = "2.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "523dc4f511e55ab87b694dc30d0f820d60906ef06413f93d4d7a1385599cc149"

[[package]]
name = "mime"
version = "0.3.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6877bb514081ee2a7ff5ef9de3281f14a4dd4bceac4c09388074a6b5df8a139a"

[[package]]
name = "miniz_oxide"
version = "0.7.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9d811f3e15f28568be3407c8e7fdb6514c1cda3cb30683f15b6a1a1dc4ea14a7"
dependencies = [
 "adler",
]

[[package]]
name = "mio"
version = "0.8.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f3d0b296e374a4e6f3c7b0a1f5a51d748a0d34c85e7dc48fc3fa9a87657fe09"
dependencies = [
 "libc",
 "wasi",
 "windows-sys 0.48.0",
]

[[package]]
name = "native-tls"
version = "0.2.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "07226173c32f2926027b63cce4bcd8076c3552846cbe7925f3aaffeac0a3b92e"
dependencies = [
 "lazy_static",
 "libc",
 "log",
 "openssl",
 "openssl-probe",
 "openssl-sys",
 "schannel",
 "security-framework",
 "security-framework-sys",
 "tempfile",
]

[[package]]
name = "num-traits"
version = "0.2.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "da0df0e5185db44f69b44f26786fe401b6c293d1907744beaa7fa62b2e5a517a"
dependencies = [
 "autocfg",
]

[[package]]
name = "num_cpus"
version = "1.16.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4161fcb6d602d4d2081af7c3a45852d875a03dd337a6bfdd6e06407b61342a43"
dependencies = [
 "hermit-abi",
 "libc",
]

[[package]]
name = "numtoa"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8f8bdf33df195859076e54ab11ee78a1b208382d3a26ec40d142ffc1ecc49ef"

[[package]]
name = "object"
version = "0.32.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a6a622008b6e321afc04970976f62ee297fdbaa6f95318ca343e3eebb9648441"
dependencies = [
 "memchr",
]

[[package]]
name = "once_cell"
version = "1.19.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3fdb12b2476b595f9358c5161aa467c2438859caa136dec86c26fdd2efe17b92"

[[package]]
name = "openssl"
version = "0.10.63"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "15c9d69dd87a29568d4d017cfe8ec518706046a05184e5aea92d0af890b803c8"
dependencies = [
 "bitflags 2.4.2",
 "cfg-if 1.0.0",
 "foreign-types",
 "libc",
 "once_cell",
 "openssl-macros",
 "openssl-sys",
]

[[package]]
name = "openssl-macros"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a948666b637a0f465e8564c73e89d4dde00d72d4d473cc972f390fc3dcee7d9c"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "openssl-probe"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ff011a302c396a5197692431fc1948019154afc178baf7d8e37367442a4601cf"

[[package]]
name = "openssl-sys"
version = "0.9.99"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "22e1bf214306098e4832460f797824c05d25aacdf896f64a985fb0fd992454ae"
dependencies = [
 "cc",
 "libc",
 "pkg-config",
 "vcpkg",
]

[[package]]
name = "percent-encoding"
version = "2.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e3148f5046208a5d56bcfc03053e3ca6334e51da8dfb19b6cdc8b306fae3283e"

[[package]]
name = "pin-project-lite"
version = "0.2.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8afb450f006bf6385ca15ef45d71d2288452bc3683ce2e2cacc0d18e4be60b58"

[[package]]
name = "pin-utils"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184"

[[package]]
name = "pkg-config"
version = "0.3.29"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2900ede94e305130c13ddd391e0ab7cbaeb783945ae07a279c268cb05109c6cb"

[[package]]
name = "proc-macro2"
version = "1.0.78"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e2422ad645d89c99f8f3e6b88a9fdeca7fabeac836b1002371c4367c8f984aae"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "quote"
version = "1.0.35"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "291ec9ab5efd934aaf503a6466c5d5251535d108ee747472c3977cc5acc868ef"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "radix64"
version = "0.6.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "999718fa65c3be3a74f3f6dae5a98526ff436ea58a82a574f0de89eecd342bee"
dependencies = [
 "arrayref",
 "cfg-if 0.1.10",
]

[[package]]
name = "reader_for_microxml"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0d726a3f4c11def37106edaf44caf861a9012ebcd4eb6f748cc4fd93c2a15de1"

[[package]]
name = "redox_syscall"
version = "0.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4722d768eff46b75989dd134e5c353f0d6296e5aaa3132e776cbdb56be7731aa"
dependencies = [
 "bitflags 1.3.2",
]

[[package]]
name = "redox_termios"
version = "0.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "20145670ba436b55d91fc92d25e71160fbfbdd57831631c8d7d36377a476f1cb"

[[package]]
name = "regex"
version = "1.10.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b62dbe01f0b06f9d8dc7d49e05a0785f153b00b2c227856282f671e0318c9b15"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-automata",
 "regex-syntax",
]

[[package]]
name = "regex-automata"
version = "0.4.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5bb987efffd3c6d0d8f5f89510bb458559eab11e4f869acb20bf845e016259cd"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-syntax",
]

[[package]]
name = "regex-syntax"
version = "0.8.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c08c74e62047bb2de4ff487b251e4a92e24f48745648451635cec7d591162d9f"

[[package]]
name = "reqwest"
version = "0.11.24"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c6920094eb85afde5e4a138be3f2de8bbdf28000f0029e72c45025a56b042251"
dependencies = [
 "base64",
 "bytes",
 "encoding_rs",
 "futures-core",
 "futures-util",
 "h2",
 "http",
 "http-body",
 "hyper",
 "hyper-tls",
 "ipnet",
 "js-sys",
 "log",
 "mime",
 "native-tls",
 "once_cell",
 "percent-encoding",
 "pin-project-lite",
 "rustls-pemfile",
 "serde",
 "serde_json",
 "serde_urlencoded",
 "sync_wrapper",
 "system-configuration",
 "tokio",
 "tokio-native-tls",
 "tower-service",
 "url",
 "wasm-bindgen",
 "wasm-bindgen-futures",
 "web-sys",
 "winreg",
]

[[package]]
name = "ring"
version = "0.17.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "688c63d65483050968b2a8937f7995f443e27041a0f7700aa59b0822aedebb74"
dependencies = [
 "cc",
 "getrandom",
 "libc",
 "spin",
 "untrusted",
 "windows-sys 0.48.0",
]

[[package]]
name = "rustc-demangle"
version = "0.1.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d626bb9dae77e28219937af045c257c28bfd3f69333c512553507f5f9798cb76"

[[package]]
name = "rustix"
version = "0.38.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6ea3e1a662af26cd7a3ba09c0297a31af215563ecf42817c98df621387f4e949"
dependencies = [
 "bitflags 2.4.2",
 "errno",
 "libc",
 "linux-raw-sys",
 "windows-sys 0.52.0",
]

[[package]]
name = "rustls-pemfile"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1c74cae0a4cf6ccbbf5f359f08efdf8ee7e1dc532573bf0db71968cb56b1448c"
dependencies = [
 "base64",
]

[[package]]
name = "ryu"
version = "1.0.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f98d2aa92eebf49b69786be48e4477826b256916e84a57ff2a4f21923b48eb4c"

[[package]]
name = "schannel"
version = "0.1.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fbc91545643bcf3a0bbb6569265615222618bdf33ce4ffbbd13c4bbd4c093534"
dependencies = [
 "windows-sys 0.52.0",
]

[[package]]
name = "security-framework"
version = "2.9.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "05b64fb303737d99b81884b2c63433e9ae28abebe5eb5045dcdd175dc2ecf4de"
dependencies = [
 "bitflags 1.3.2",
 "core-foundation",
 "core-foundation-sys",
 "libc",
 "security-framework-sys",
]

[[package]]
name = "security-framework-sys"
version = "2.9.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e932934257d3b408ed8f30db49d85ea163bfe74961f017f405b025af298f0c7a"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "semver"
version = "1.0.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b97ed7a9823b74f99c7742f5336af7be5ecd3eeafcb1507d1fa93347b1d589b0"

[[package]]
name = "serde"
version = "1.0.196"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "870026e60fa08c69f064aa766c10f10b1d62db9ccd4d0abb206472bee0ce3b32"
dependencies = [
 "serde_derive",
]

[[package]]
name = "serde_derive"
version = "1.0.196"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "33c85360c95e7d137454dc81d9a4ed2b8efd8fbe19cee57357b32b9771fccb67"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "serde_json"
version = "1.0.113"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "69801b70b1c3dac963ecb03a364ba0ceda9cf60c71cfe475e99864759c8b8a79"
dependencies = [
 "itoa",
 "ryu",
 "serde",
]

[[package]]
name = "serde_spanned"
version = "0.6.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eb3622f419d1296904700073ea6cc23ad690adbd66f13ea683df73298736f0c1"
dependencies = [
 "serde",
]

[[package]]
name = "serde_urlencoded"
version = "0.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d3491c14715ca2294c4d6a88f15e84739788c1d030eed8c110436aafdaa2f3fd"
dependencies = [
 "form_urlencoded",
 "itoa",
 "ryu",
 "serde",
]

[[package]]
name = "sha2"
version = "0.10.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "793db75ad2bcafc3ffa7c68b215fee268f537982cd901d132f89c6343f3a3dc8"
dependencies = [
 "cfg-if 1.0.0",
 "cpufeatures",
 "digest",
]

[[package]]
name = "slab"
version = "0.4.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f92a496fb766b417c996b9c5e57daf2f7ad3b0bebe1ccfca4856390e3d3bb67"
dependencies = [
 "autocfg",
]

[[package]]
name = "socket2"
version = "0.5.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7b5fac59a5cb5dd637972e5fca70daf0523c9067fcdc4842f053dae04a18f8e9"
dependencies = [
 "libc",
 "windows-sys 0.48.0",
]

[[package]]
name = "spin"
version = "0.9.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6980e8d7511241f8acf4aebddbb1ff938df5eebe98691418c4468d0b72a96a67"

[[package]]
name = "syn"
version = "2.0.48"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0f3531638e407dfc0814761abb7c00a5b54992b849452a0646b7f65c9f770f3f"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-ident",
]

[[package]]
name = "sync_wrapper"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2047c6ded9c721764247e62cd3b03c09ffc529b2ba5b10ec482ae507a4a70160"

[[package]]
name = "system-configuration"
version = "0.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ba3a3adc5c275d719af8cb4272ea1c4a6d668a777f37e115f6d11ddbc1c8e0e7"
dependencies = [
 "bitflags 1.3.2",
 "core-foundation",
 "system-configuration-sys",
]

[[package]]
name = "system-configuration-sys"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a75fb188eb626b924683e3b95e3a48e63551fcfb51949de2f06a9d91dbee93c9"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "tempfile"
version = "3.10.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a365e8cd18e44762ef95d87f284f4b5cd04107fec2ff3052bd6a3e6069669e67"
dependencies = [
 "cfg-if 1.0.0",
 "fastrand",
 "rustix",
 "windows-sys 0.52.0",
]

[[package]]
name = "termion"
version = "3.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "417813675a504dfbbf21bfde32c03e5bf9f2413999962b479023c02848c1c7a5"
dependencies = [
 "libc",
 "libredox",
 "numtoa",
 "redox_termios",
]

[[package]]
name = "thiserror"
version = "1.0.56"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d54378c645627613241d077a3a79db965db602882668f9136ac42af9ecb730ad"
dependencies = [
 "thiserror-impl",
]

[[package]]
name = "thiserror-impl"
version = "1.0.56"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fa0faa943b50f3db30a20aa7e265dbc66076993efed8463e8de414e5d06d3471"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "tinyvec"
version = "1.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50"
dependencies = [
 "tinyvec_macros",
]

[[package]]
name = "tinyvec_macros"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20"

[[package]]
name = "tokio"
version = "1.36.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "61285f6515fa018fb2d1e46eb21223fff441ee8db5d0f1435e8ab4f5cdb80931"
dependencies = [
 "backtrace",
 "bytes",
 "libc",
 "mio",
 "num_cpus",
 "pin-project-lite",
 "socket2",
 "windows-sys 0.48.0",
]

[[package]]
name = "tokio-native-tls"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bbae76ab933c85776efabc971569dd6119c580d8f5d448769dec1764bf796ef2"
dependencies = [
 "native-tls",
 "tokio",
]

[[package]]
name = "tokio-util"
version = "0.7.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5419f34732d9eb6ee4c3578b7989078579b7f039cbbb9ca2c4da015749371e15"
dependencies = [
 "bytes",
 "futures-core",
 "futures-sink",
 "pin-project-lite",
 "tokio",
 "tracing",
]

[[package]]
name = "toml"
version = "0.8.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a9aad4a3066010876e8dcf5a8a06e70a558751117a145c6ce2b82c2e2054290"
dependencies = [
 "serde",
 "serde_spanned",
 "toml_datetime",
 "toml_edit",
]

[[package]]
name = "toml_datetime"
version = "0.6.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3550f4e9685620ac18a50ed434eb3aec30db8ba93b0287467bca5826ea25baf1"
dependencies = [
 "serde",
]

[[package]]
name = "toml_edit"
version = "0.22.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0c9ffdf896f8daaabf9b66ba8e77ea1ed5ed0f72821b398aba62352e95062951"
dependencies = [
 "indexmap",
 "serde",
 "serde_spanned",
 "toml_datetime",
 "winnow",
]

[[package]]
name = "tower-service"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6bc1c9ce2b5135ac7f93c72918fc37feb872bdc6a5533a8b85eb4b86bfdae52"

[[package]]
name = "tracing"
version = "0.1.40"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c3523ab5a71916ccf420eebdf5521fcef02141234bbc0b8a49f2fdc4544364ef"
dependencies = [
 "pin-project-lite",
 "tracing-core",
]

[[package]]
name = "tracing-core"
version = "0.1.32"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c06d3da6113f116aaee68e4d601191614c9053067f9ab7f6edbcb161237daa54"
dependencies = [
 "once_cell",
]

[[package]]
name = "try-lock"
version = "0.2.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e421abadd41a4225275504ea4d6566923418b7f05506fbc9c0fe86ba7396114b"

[[package]]
name = "typenum"
version = "1.17.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "42ff0bf0c66b8238c6f3b578df37d0b7848e55df8577b3f74f92a69acceeb825"

[[package]]
name = "unicode-bidi"
version = "0.3.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "08f95100a766bf4f8f28f90d77e0a5461bbdb219042e7679bebe79004fed8d75"

[[package]]
name = "unicode-ident"
version = "1.0.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3354b9ac3fae1ff6755cb6db53683adb661634f67557942dea4facebec0fee4b"

[[package]]
name = "unicode-normalization"
version = "0.1.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c5713f0fc4b5db668a2ac63cdb7bb4469d8c9fed047b1d0292cc7b0ce2ba921"
dependencies = [
 "tinyvec",
]

[[package]]
name = "untrusted"
version = "0.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8ecb6da28b8a351d773b68d5825ac39017e680750f980f3a1a85cd8dd28a47c1"

[[package]]
name = "unwrap"
version = "1.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7e33648dd74328e622c7be51f3b40a303c63f93e6fa5f08778b6203a4c25c20f"

[[package]]
name = "url"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "31e6302e3bb753d46e83516cae55ae196fc0c309407cf11ab35cc51a4c2a4633"
dependencies = [
 "form_urlencoded",
 "idna",
 "percent-encoding",
]

[[package]]
name = "vcpkg"
version = "0.2.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "accd4ea62f7bb7a82fe23066fb0957d48ef677f6eeb8215f372f52e48bb32426"

[[package]]
name = "version_check"
version = "0.9.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f"

[[package]]
name = "want"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bfa7760aed19e106de2c7c0b581b509f2f25d3dacaf737cb82ac61bc6d760b0e"
dependencies = [
 "try-lock",
]

[[package]]
name = "wasi"
version = "0.11.0+wasi-snapshot-preview1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9c8d87e72b64a3b4db28d11ce29237c246188f4f51057d65a7eab63b7987e423"

[[package]]
name = "wasm-bindgen"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c1e124130aee3fb58c5bdd6b639a0509486b0338acaaae0c84a5124b0f588b7f"
dependencies = [
 "cfg-if 1.0.0",
 "wasm-bindgen-macro",
]

[[package]]
name = "wasm-bindgen-backend"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c9e7e1900c352b609c8488ad12639a311045f40a35491fb69ba8c12f758af70b"
dependencies = [
 "bumpalo",
 "log",
 "once_cell",
 "proc-macro2",
 "quote",
 "syn",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-futures"
version = "0.4.41"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "877b9c3f61ceea0e56331985743b13f3d25c406a7098d45180fb5f09bc19ed97"
dependencies = [
 "cfg-if 1.0.0",
 "js-sys",
 "wasm-bindgen",
 "web-sys",
]

[[package]]
name = "wasm-bindgen-macro"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b30af9e2d358182b5c7449424f017eba305ed32a7010509ede96cdc4696c46ed"
dependencies = [
 "quote",
 "wasm-bindgen-macro-support",
]

[[package]]
name = "wasm-bindgen-macro-support"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "642f325be6301eb8107a83d12a8ac6c1e1c54345a7ef1a9261962dfefda09e66"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
 "wasm-bindgen-backend",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-shared"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4f186bd2dcf04330886ce82d6f33dd75a7bfcf69ecf5763b89fcde53b6ac9838"

[[package]]
name = "web-sys"
version = "0.3.68"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "96565907687f7aceb35bc5fc03770a8a0471d82e479f25832f54a0e3f4b28446"
dependencies = [
 "js-sys",
 "wasm-bindgen",
]

[[package]]
name = "windows-core"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "33ab640c8d7e35bf8ba19b884ba838ceb4fba93a4e8c65a9059d08afcfc683d9"
dependencies = [
 "windows-targets 0.52.0",
]

[[package]]
name = "windows-sys"
version = "0.48.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "677d2418bec65e3338edb076e806bc1ec15693c5d0104683f2efe857f61056a9"
dependencies = [
 "windows-targets 0.48.5",
]

[[package]]
name = "windows-sys"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d"
dependencies = [
 "windows-targets 0.52.0",
]

[[package]]
name = "windows-targets"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a2fa6e2155d7247be68c096456083145c183cbbbc2764150dda45a87197940c"
dependencies = [
 "windows_aarch64_gnullvm 0.48.5",
 "windows_aarch64_msvc 0.48.5",
 "windows_i686_gnu 0.48.5",
 "windows_i686_msvc 0.48.5",
 "windows_x86_64_gnu 0.48.5",
 "windows_x86_64_gnullvm 0.48.5",
 "windows_x86_64_msvc 0.48.5",
]

[[package]]
name = "windows-targets"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8a18201040b24831fbb9e4eb208f8892e1f50a37feb53cc7ff887feb8f50e7cd"
dependencies = [
 "windows_aarch64_gnullvm 0.52.0",
 "windows_aarch64_msvc 0.52.0",
 "windows_i686_gnu 0.52.0",
 "windows_i686_msvc 0.52.0",
 "windows_x86_64_gnu 0.52.0",
 "windows_x86_64_gnullvm 0.52.0",
 "windows_x86_64_msvc 0.52.0",
]

[[package]]
name = "windows_aarch64_gnullvm"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2b38e32f0abccf9987a4e3079dfb67dcd799fb61361e53e2882c3cbaf0d905d8"

[[package]]
name = "windows_aarch64_gnullvm"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cb7764e35d4db8a7921e09562a0304bf2f93e0a51bfccee0bd0bb0b666b015ea"

[[package]]
name = "windows_aarch64_msvc"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dc35310971f3b2dbbf3f0690a219f40e2d9afcf64f9ab7cc1be722937c26b4bc"

[[package]]
name = "windows_aarch64_msvc"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bbaa0368d4f1d2aaefc55b6fcfee13f41544ddf36801e793edbbfd7d7df075ef"

[[package]]
name = "windows_i686_gnu"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a75915e7def60c94dcef72200b9a8e58e5091744960da64ec734a6c6e9b3743e"

[[package]]
name = "windows_i686_gnu"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a28637cb1fa3560a16915793afb20081aba2c92ee8af57b4d5f28e4b3e7df313"

[[package]]
name = "windows_i686_msvc"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f55c233f70c4b27f66c523580f78f1004e8b5a8b659e05a4eb49d4166cca406"

[[package]]
name = "windows_i686_msvc"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ffe5e8e31046ce6230cc7215707b816e339ff4d4d67c65dffa206fd0f7aa7b9a"

[[package]]
name = "windows_x86_64_gnu"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "53d40abd2583d23e4718fddf1ebec84dbff8381c07cae67ff7768bbf19c6718e"

[[package]]
name = "windows_x86_64_gnu"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3d6fa32db2bc4a2f5abeacf2b69f7992cd09dca97498da74a151a3132c26befd"

[[package]]
name = "windows_x86_64_gnullvm"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0b7b52767868a23d5bab768e390dc5f5c55825b6d30b86c844ff2dc7414044cc"

[[package]]
name = "windows_x86_64_gnullvm"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1a657e1e9d3f514745a572a6846d3c7aa7dbe1658c056ed9c3344c4109a6949e"

[[package]]
name = "windows_x86_64_msvc"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ed94fce61571a4006852b7389a063ab983c02eb1bb37b47f8272ce92d06d9538"

[[package]]
name = "windows_x86_64_msvc"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dff9641d1cd4be8d1a070daf9e3773c5f67e78b4d9d42263020c057706765c04"

[[package]]
name = "winnow"
version = "0.5.39"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5389a154b01683d28c77f8f68f49dea75f0a4da32557a58f68ee51ebba472d29"
dependencies = [
 "memchr",
]

[[package]]
name = "winreg"
version = "0.50.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "524e57b2c537c0f9b1e69f1965311ec12182b4122e45035b1508cd24d2adadb1"
dependencies = [
 "cfg-if 1.0.0",
 "windows-sys 0.48.0",
]
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "LICENSE",
        file_content: r###"MIT License

Copyright (c) 2024 bestia.dev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"###,
    });
    // endregion: files copied into strings by automation tasks

    // return
    vec_file
}
