use std::{error::Error, fmt};

#[derive(Debug)]
pub enum GitRunnerError {
    NotAGitRepository,
    IoError(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
}
impl fmt::Display for GitRunnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GitRunnerError::NotAGitRepository => write!(f, "Not inside a Git work tree."),
            GitRunnerError::IoError(e) => write!(f, "I/O error: {}", e),
            GitRunnerError::Utf8Error(e) => write!(f, "UTF-8 decoding error: {}", e),
        }
    }
}

impl Error for GitRunnerError {}

impl From<std::io::Error> for GitRunnerError {
    fn from(err: std::io::Error) -> GitRunnerError {
        GitRunnerError::IoError(err)
    }
}

impl From<std::string::FromUtf8Error> for GitRunnerError {
    fn from(err: std::string::FromUtf8Error) -> GitRunnerError {
        GitRunnerError::Utf8Error(err)
    }
}