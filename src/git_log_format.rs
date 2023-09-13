pub enum GitLogFormat {
    Compact,
    Full,
}

impl GitLogFormat {
    pub fn to_git_arg(&self) -> &str {
        match self {
            GitLogFormat::Compact => "--pretty=format:%s",
            GitLogFormat::Full => "--pretty=format:%H%n%an%n%ad%n%s",
        }
    }
}
