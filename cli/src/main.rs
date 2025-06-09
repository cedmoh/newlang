use clap::{Arg, Command, command};
use core::{eval::Value, runtime::Runtime};
use std::path::PathBuf;

mod repl;
mod styles;

use styles::CARGO_STYLING;

use crate::repl::Repl;

fn main() {
    let command = command!()
        .styles(CARGO_STYLING)
        .about("A CLI tool for working with projects written in newlang")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Run the script pointed to by the given path.")
                .value_parser(clap::value_parser!(PathBuf))
                .required(false)
                .index(1),
        )
        .subcommand(
            Command::new("fmt")
                .about("Format files in the given path recursively.")
                .arg(
                    Arg::new("path")
                        .help("Path to format recursively.")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                ),
        );

    let matches = command.get_matches();

    match matches.subcommand() {
        // No subcommand provided.
        None => {
            if let Some(file) = matches.get_one::<PathBuf>("file") {
                println!("{}", execute_file(file));
            } else {
                repl();
            }
        }
        // Provided Subcommand is `fmt`
        Some(("fmt", sub_m)) => {
            let path: &PathBuf = sub_m.get_one("path").unwrap();
            format_path(path);
        }
        _ => {
            unreachable!("Any unhandled argument was assumed to be handled as path to file.")
        }
    }
}

fn execute_file(path: &PathBuf) -> Value {
    // Read file.
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    // Execute contents.
    Runtime::new().run(content.as_str())
}

fn format_path(path: &PathBuf) {
    println!("Formatting files at path: {:?}", path);
    todo!()
}

fn repl() {
    Repl::new().start();
}
