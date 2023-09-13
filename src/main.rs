mod git_log_format;
use git_log_format::GitLogFormat;

use clap::{App, Arg};
use std::env;
use std::io::Result;
use std::process::Command;
use std::str;

fn parse_args() -> (GitLogFormat, Option<String>, Option<String>) {
    let matches = App::new("ReleaseCraftman")
        .version("1.0")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .help("Sets the format of git log output (compact|hard)"),
        )
        .arg(
            Arg::with_name("start_tag")
                .short("s")
                .long("start")
                .takes_value(true)
                .help("Starting tag for the git log"),
        )
        .arg(
            Arg::with_name("end_tag")
                .short("e")
                .long("end")
                .takes_value(true)
                .help("Ending tag for the git log"),
        )
        .get_matches();

    let log_format = match matches.value_of("format").unwrap_or("compact") {
        "compact" => GitLogFormat::Compact,
        "hard" => GitLogFormat::Full,
        _ => GitLogFormat::Compact,
    };

    let start_tag = matches.value_of("start_tag").map(|s| s.to_string());
    let end_tag = matches.value_of("end_tag").map(|e| e.to_string());

    (log_format, start_tag, end_tag)
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

fn execute_git_log(
    format: &GitLogFormat,
    repo_path: &String,
    start_tag: &Option<String>,
    end_tag: &Option<String>,
) -> Result<String> {
    let mut command = Command::new("git");
    command.arg("log").arg(format.to_git_arg());

    if let (Some(start), Some(end)) = (start_tag, end_tag) {
        command.arg(format!("{}..{}", start, end));
    } else if let Some(start) = start_tag {
        command.arg(format!("{}..", start));
    } else if let Some(end) = end_tag {
        command.arg(format!("..{}", end));
    }

    let output = command.current_dir(repo_path).output()?;

    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(output_str)
}

fn print_commits(commits: Vec<&str>) {
    for (_i, commit) in commits.iter().enumerate() {
        println!("{}", commit);
    }
}

fn main() -> Result<()> {
    let (log_format, start_tag, end_tag) = parse_args();
    let repo_path = get_repo_path()?;

    println!("Executing git log command in directory: {}", repo_path);

    let output_str = execute_git_log(&log_format, &repo_path, &start_tag, &end_tag)?;
    let commits: Vec<&str> = output_str.lines().collect();

    print_commits(commits);

    Ok(())
}
