use clap::{Arg, Command, command};
use core::{eval::Value, runner::run};
use std::path::PathBuf;

mod styles;

use styles::CARGO_STYLING;

fn main() {
    let command = command!()
        .styles(CARGO_STYLING)
        .about("A CLI tool for working with projects written in newlang")
        .subcommand_required(false)
        .arg_required_else_help(true)
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
                match execute_file(file) {
                    Some(val) => println!("{}", val),
                    None => println!(""),
                }
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

fn execute_file(path: &PathBuf) -> Option<Value> {
    // Read file.
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    // Execute contents.
    run(content.as_str()).expect("Failed to execute file.")
}

fn format_path(path: &PathBuf) {
    println!("Formatting files at path: {:?}", path);
    todo!()
}
