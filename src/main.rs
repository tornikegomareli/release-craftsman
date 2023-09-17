mod filereader;
mod argparser;
mod gitrunner;
mod git_log_format;
mod gptrunner;

use filereader::file_reader::{FileReader, PromptType};
use argparser::arg_parser::ArgParser;
use std::str;
use gitrunner::git_runner::GitRunner;
use gptrunner::gpt_runner::ChatGptRunner;
use std::process;

use crate::gitrunner::git_runner_error::GitRunnerError;


fn print_commits(commits: Vec<&str>) {
    for (_i, commit) in commits.iter().enumerate() {
        println!("{}", commit);
    }
}

#[tokio::main]
async fn main() -> Result<(), GitRunnerError> {
    // Parsing command line arguments and getting repo path
    let (log_format, start_tag, end_tag, api_key, model, version) = ArgParser::parse_args();
    let mut output_str: String = String::new();
    match GitRunner::get_repo_path() {
        Ok(repo_path) => {
            let repo_path = GitRunner::get_repo_path()?;

            println!("Executing git log command in directory: {}", repo_path);

            // Fetching git logs
            output_str = GitRunner::execute_git_log(&log_format, &repo_path, &start_tag, &end_tag)?;
        },
        Err(err) => {
            eprintln!("👷🏻‍ An error occurred ❌: {}", err);
            process::exit(1);
        }
    }


    let commits: Vec<&str> = output_str.lines().collect();

    // Print commit logs (For debugging)
    print_commits(commits.clone());

    let fr = FileReader::new();
    let version_str = version.unwrap_or_else(|| "1.0.0".to_string());


    // Convert commit logs Vec<&str> to a single String
    let commit_logs = commits.join("\n");

    // Choose a prompt type based on some criteria (here we're using GeneralMarkdown for demonstration)
    // dynamically select this based on user input or other conditions
    let prompt_type = PromptType::GeneralMarkdown;

    // Generate the final prompt string
    let final_prompt = fr.read_and_replace(prompt_type, &version_str, &commit_logs)?;

    // Output the final prompt
    println!("Final Prompt: {}", final_prompt);


    if let Some(api_key) = api_key {
        let base_url = "https://api.openai.com";
        let gpt_response = ChatGptRunner::run_chat_gpt(&api_key, model, &final_prompt).await?;
        println!("GPT response {}", gpt_response);
    }

    Ok(())
}