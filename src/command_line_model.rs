use crate::git_log_format::GitLogFormat;
use chat_gpt_lib_rs::models::Model;
pub struct CommandLineModel {
    pub log_format: GitLogFormat,
    pub start_tag: Option<String>,
    pub end_tag: Option<String>,
    pub api_key: Option<String>,
    pub gpt_model: Model,
    pub version: Option<String>,
}