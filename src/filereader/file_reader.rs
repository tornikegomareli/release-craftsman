use include_dir::{include_dir, Dir};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
pub enum PromptType {
    GeneralMarkdown,
    GeneralJSON,
    Detailed,
}

impl PromptType {
    fn file_name(&self) -> &'static str {
        match self {
            PromptType::GeneralMarkdown => "general_release_notes_markdown.txt",
            PromptType::GeneralJSON => "general_release_notes_json.txt",
            PromptType::Detailed => "detailed_release_notes.txt",
        }
    }
}

pub struct FileReader {}

impl FileReader {
    pub fn new() -> FileReader {
        FileReader {}
    }

    pub fn read_and_replace(
        &self,
        prompt_type: PromptType,
        version: &str,
        commit_logs: &str,
    ) -> io::Result<String> {
        const PROMPTS_DIR: Dir = include_dir!("src/prompts");
        let file = PROMPTS_DIR
            .get_file(prompt_type.file_name())
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "File not found"))?;

        let content_str = std::str::from_utf8(file.contents())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Replace placeholders
        let content_replaced_version = content_str.replace("&VERSION&", version);
        let content_replaced_all = content_replaced_version.replace("&COMMIT_LOGS&", commit_logs);

        Ok(content_replaced_all)
    }
}
