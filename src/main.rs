mod git_log_format;
use git_log_format::GitLogFormat;

mod file_reader;
use file_reader::FileReader;
use file_reader::PromptType;

use clap::{App, Arg};
use std::env;
use std::io::Result;
use std::process::Command;
use std::str;
use include_dir::{include_dir, Dir};

use chat_gpt_lib_rs::{ChatGPTClient, ChatInput, Message, Model, Role};
use clap::Format::Error;

fn parse_args() -> (GitLogFormat, Option<String>, Option<String>, Option<String>, Model, Option<String>) {
    let matches = App::new("releasecraftsman")
        .version("0.1.1")
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
        .arg(
            Arg::with_name("api_key")
                .short("k")
                .long("key")
                .takes_value(true)
                .help("API Key for GPT"),
        )
        .arg(
            Arg::with_name("model")
                .short("m")
                .long("model")
                .takes_value(true)
                .default_value("Gpt_3_5Turbo")
                .help("GPT model to use (Gpt_3_5Turbo|Gpt_4)"),
        )
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .takes_value(true)
                .help("Version of the release")
        )
        .get_matches();

    let log_format = match matches.value_of("format").unwrap_or("compact") {
        "compact" => GitLogFormat::Compact,
        "hard" => GitLogFormat::Full,
        _ => GitLogFormat::Compact,
    };

    let start_tag = matches.value_of("start_tag").map(|s| s.to_string());
    let end_tag = matches.value_of("end_tag").map(|e| e.to_string());

    let api_key = matches.value_of("api_key").map(|s| s.to_string());
    let model = match matches.value_of("model").unwrap() {
        "Gpt_4" => Model::Gpt_4,
        "Gpt_3_5Turbo" => Model::Gpt3_5Turbo,
        _ => Model::Gpt3_5Turbo,
    };

    let version = matches.value_of("version").map(|v| v.to_string());

    (log_format, start_tag, end_tag, api_key, model, version)
}

fn get_repo_path() -> Result<String> {
    let repo_path = if cfg!(debug_assertions) {
        println!("### Running Debug mode ### ");
        String::from("/Users/tornike-mac/Development/Composable2048")
    } else {
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

#[tokio::main]
async fn main() -> Result<()> {
    // Parsing command line arguments and getting repo path
    let (log_format, start_tag, end_tag, api_key, model, version) = parse_args();
    let repo_path = get_repo_path()?;

    println!("Executing git log command in directory: {}", repo_path);

    // Fetching git logs
    let output_str = execute_git_log(&log_format, &repo_path, &start_tag, &end_tag)?;
    let commits: Vec<&str> = output_str.lines().collect();

    // Print commit logs (For debugging)
    print_commits(commits.clone());

    let fr = FileReader::new();
    let version_str = version.unwrap_or_else(|| "1.0.0".to_string()); // Replace with your real version if available


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
        let gpt_response = run_chat_gpt(&api_key, model, &final_prompt).await?;
        println!("GPT response {}", gpt_response);
    }

    Ok(())
}

async fn run_chat_gpt(api_key: &str, model: Model, message: &str) -> Result<String> {
    let base_url = "https://api.openai.com";
    let client = ChatGPTClient::new(api_key, base_url);
    let chat_input = ChatInput { model, messages: vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: Role::User,
            content: message.to_string(),
        },
    ],
        ..Default::default()
    };

    let response = client.chat(chat_input).await.unwrap();
    Ok(response.choices[0].message.content.clone())
}
