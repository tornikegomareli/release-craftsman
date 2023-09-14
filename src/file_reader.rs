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

    /// Reads a prompt file and replaces placeholders with actual data.
    ///
    /// # Arguments
    ///
    /// * `prompt_type` - An enum representing the type of prompt
    /// * `version` - A string slice containing the version to insert into the prompt
    /// * `commit_logs` - A string slice containing the commit logs to insert into the prompt
    ///
    /// # Example
    ///
    /// ```
    /// let fr = FileReader::new();
    /// let prompt = fr.read_and_replace(PromptType::GeneralMarkdown, "v1.2.3", "My commit logs");
    /// ```
    pub fn read_and_replace(
        &self,
        prompt_type: PromptType,
        version: &str,
        commit_logs: &str,
    ) -> io::Result<String> {
        // Open file
        let path = Path::new(prompt_type.file_name());
        let file = File::open(&path)?;

        // Read file into a string
        let reader = BufReader::new(file);
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line?);
            content.push('\n');
        }

        // Replace placeholders
        let content_replaced_version = content.replace("&VERSION&", version);
        let content_replaced_all = content_replaced_version.replace("&COMMIT_LOGS&", commit_logs);

        Ok(content_replaced_all)
    }
}
