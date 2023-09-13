mod git_log_format;
use git_log_format::GitLogFormat;

use clap::{App, Arg};
use std::env;
use std::process::Command;
use std::str;

fn main() {
    let matches = App::new("ReleaseCraftman")
        .version("1.0")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .help("Sets the format of git log output (compact|hard)"),
        )
        .get_matches();

    let log_format = match matches.value_of("format").unwrap_or("compact") {
        "compact" => GitLogFormat::Compact,
        "hard" => GitLogFormat::Full,
        _ => {
            eprintln!("Invalid format specified. Use 'compact' or 'hard'.");
            return;
        }
    };

    let repo_path: String;

    #[cfg(debug_assertions)]
    {
        println!("Running in debug mode");
        // Testing purpose
        repo_path = String::from("/Users/tornike-mac/Development/Composable2048");
    }

    #[cfg(not(debug_assertions))]
    {
        println!("Running in release mode");
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Error fetching current directory: {}", e);
                return;
            }
        };

        repo_path = match current_dir.to_str() {
            Some(path) => String::from(path),
            None => {
                eprintln!("Error converting path to string");
                return;
            }
        };
    }

    println!("Executing git log command in directory: {}", repo_path);
    let output = match Command::new("git")
        .arg("log")
        .arg(log_format.to_git_arg())
        .arg("-n")
        .arg("5")
        .current_dir(&repo_path)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to execute git command: {}", e);
            return;
        }
    };

    let output_str = match str::from_utf8(&output.stdout) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Could not convert to UTF-8: {}", e);
            return;
        }
    };

    let commits: Vec<&str> = output_str.lines().collect();
    for (_i, commit) in commits.iter().enumerate() {
        println!("{}", commit);
    }
}
