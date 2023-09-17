use std::env;
use std::process::Command;
use crate::git_log_format::GitLogFormat;
use crate::gitrunner::git_runner_error::GitRunnerError;

pub struct GitRunner {}

impl GitRunner {
    pub fn get_repo_path() -> Result<String, GitRunnerError> {
        let is_git_repo = Command::new("git")
            .args(&["rev-parse", "--is-inside-work-tree"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if !is_git_repo {
            return Err(GitRunnerError::NotAGitRepository);
        }

        let repo_path = if cfg!(debug_assertions) {
            println!("### Running Debug mode ### ");
            String::from("/Users/tornike-mac/Development/Composable2048")
        } else {
            let current_dir = env::current_dir()?;
            current_dir.to_str().unwrap().to_string()
        };

        Ok(repo_path)
    }

    pub fn execute_git_log(
        format: &GitLogFormat,
        repo_path: &String,
        start_tag: &Option<String>,
        end_tag: &Option<String>,
    ) -> Result<String, GitRunnerError> {
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

        let output_str = String::from_utf8(output.stdout)?;

        Ok(output_str)
    }
}