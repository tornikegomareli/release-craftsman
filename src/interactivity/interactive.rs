use crate::git_log_format::GitLogFormat;
use crate::command_line_model::CommandLineModel;

use chat_gpt_lib_rs::models::Model;
use inquire::{Select, Text, Password, PasswordDisplayMode};
use std::process;
use log::log;

pub struct Interactive {
    pub cmd_model: CommandLineModel,
}
impl Interactive {
    pub fn new() -> Self {
        Interactive {
            cmd_model: CommandLineModel {
                log_format: GitLogFormat::Compact,
                start_tag: None,
                end_tag: None,
                api_key: None,
                gpt_model: Model::Gpt3_5Turbo,
                version: None,
            },
        }
    }

    pub fn run(&mut self) {
        let mod_options = vec!["Compact", "Full"];
        let gpt_mod_options = vec!["GPT3_5Turbo", "GPT4"];

        let git_log_mode = Select::new("How do you want to parse your commit logs?", mod_options).prompt();
        let log_format = match git_log_mode.unwrap() {
            "Compact" => GitLogFormat::Compact,
            "Full" => GitLogFormat::Full,
            _ => process::exit(1),
        };

        let api_key = Password::new("OpenAI API KEY: ")
            .with_display_mode(PasswordDisplayMode::Masked)
            .without_confirmation()
            .with_help_message("To use ChatGPT, enter your OpenAI API_KEY https://help.openai.com/en/articles/4936850-where-do-i-find-my-secret-api-key")
            .prompt();

        let gpt_mod = Select::new("Which GPT model you want to use?", gpt_mod_options).prompt();
        let gpt_model = match gpt_mod.unwrap() {
            "Gpt_4" => Model::Gpt_4,
            "Gpt_3_5Turbo" => Model::Gpt3_5Turbo,
            _ => Model::Gpt3_5Turbo,
        };

        let start_tag = Text::new("Tag: ")
            .with_help_message("From which tag you want to generate release notes?")
            .prompt();

        let version = Text::new("Version: ")
            .with_help_message("What is your next release version?")
            .prompt();

        self.cmd_model.log_format = log_format;
        self.cmd_model.api_key = Some(api_key.unwrap());
        self.cmd_model.gpt_model = gpt_model;
        self.cmd_model.start_tag = Some(start_tag.unwrap());
        self.cmd_model.version = Some(version.unwrap());
    }
}
