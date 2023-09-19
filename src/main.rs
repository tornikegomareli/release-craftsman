mod argparser;
mod filereader;
mod git_log_format;
mod gitrunner;
mod gptrunner;
mod command_line_model;
mod interactivity;

use std::fs::OpenOptions;
use argparser::arg_parser::ArgParser;
use filereader::file_reader::{FileReader, PromptType};
use gitrunner::git_runner::GitRunner;
use gptrunner::gpt_runner::ChatGptRunner;
use std::process;
use std::str;
use std::path::Path;
use std::io::Write;
use std::env;
use colored::*;

use crate::gitrunner::git_runner_error::GitRunnerError;
use crate::interactivity::interactive::Interactive;

fn print_commits(commits: Vec<&str>) {
    for (_i, commit) in commits.iter().enumerate() {
        println!("{}", commit);
    }
}

fn get_unique_filename(base: &str, extension: &str) -> String {
    let mut counter = 0;
    let mut filename = format!("{}{}", base, extension);

    while Path::new(&filename).exists() {
        filename = format!("{}-{}{}", base, counter, extension);
        counter += 1;
    }

    filename
}

#[tokio::main]
async fn main() -> Result<(), GitRunnerError> {
    let args: Vec<String> = env::args().collect();
    let args_provided = args.len() > 1;

    let (log_format, start_tag, end_tag, api_key, model, version) = if args_provided {
        ArgParser::parse_args()
    } else {
        let mut interactive = Interactive::new();
        interactive.run();
        let model = interactive.cmd_model;

        (
            model.log_format,
            model.start_tag,
            None,
            model.api_key,
            model.gpt_model,
            model.version
        )
    };

    let mut output_str: String = String::new();
    let mut git_repo_path: String = String::new();

    match GitRunner::get_repo_path() {
        Ok(repo_path) => {
            println!("Executing git log command in directory: {}", repo_path);
            git_repo_path = repo_path;
            // Fetching git logs
            output_str = GitRunner::execute_git_log(&log_format, &git_repo_path, &start_tag, &end_tag)?;
        }
        Err(err) => {
            eprintln!("👷🏻‍ An error occurred ❌: {}", err);
            process::exit(1);
        }
    }

    let commits: Vec<&str> = output_str.lines().collect();

    print_commits(commits.clone());

    let fr = FileReader::new();
    let version_str = version.unwrap_or_else(|| "1.0.0".to_string());

    let commit_logs = commits.join("\n");

    let prompt_type = PromptType::GeneralMarkdown;

    let final_prompt = fr.read_and_replace(prompt_type, &version_str, &commit_logs)?;

    println!("Final Prompt: {}", final_prompt);

    if let Some(api_key) = api_key {
        let gpt_response = ChatGptRunner::run_chat_gpt(&api_key, model, &final_prompt).await?;

        let unique_filename_for_changelog = get_unique_filename("RELEASE_NOTE", ".md");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&unique_filename_for_changelog)?;

        match writeln!(file, "{}", gpt_response) {
            Ok(_) => {
                let msg = format!(
                    "Change log created at: {}",
                    git_repo_path
                );

                println!("{}", msg.green());
            },
            Err(e) => {
                let msg = "Failed while creating markdown file with generated content. Try again or check `releasecraftsman --help`";
                println!("{}", msg.red());
            },
        }
    }
    Ok(())
}