mod git_log_format;
use git_log_format::GitLogFormat;

use clap::{App, Arg};
use std::env;
use std::io::Result;
use std::process::Command;
use std::str;

fn parse_args() -> GitLogFormat {
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

    match matches.value_of("format").unwrap_or("compact") {
        "compact" => GitLogFormat::Compact,
        "hard" => GitLogFormat::Full,
        _ => {
            eprintln!("Invalid format specified. Using 'compact' as default.");
            GitLogFormat::Compact
        }
    }
}

fn get_repo_path() -> Result<String> {
    let repo_path = if cfg!(debug_assertions) {
        println!("Running in debug mode");
        String::from("/Users/tornike-mac/Development/Composable2048")
    } else {
        println!("Running in release mode");
        let current_dir = env::current_dir()?;
        current_dir.to_str().unwrap().to_string()
    };

    Ok(repo_path)
}

fn execute_git_log(format: &GitLogFormat, repo_path: &String) -> Result<String> {
    let output = Command::new("git")
        .arg("log")
        .arg(format.to_git_arg())
        .arg("-n")
        .arg("5")
        .current_dir(repo_path)
        .output()?;

    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(output_str)
}

fn print_commits(commits: Vec<&str>) {
    for (i, commit) in commits.iter().enumerate() {
        println!("Commit {}: {}", i + 1, commit);
    }
}

fn main() -> Result<()> {
    let log_format = parse_args();
    let repo_path = get_repo_path()?;

    println!("Executing git log command in directory: {}", repo_path);

    let output_str = execute_git_log(&log_format, &repo_path)?;
    let commits: Vec<&str> = output_str.lines().collect();

    print_commits(commits);

    Ok(())
}
