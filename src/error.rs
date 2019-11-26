use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Unable to access current working directory")]
    WorkingDirectoryInaccessible,
    #[error("Unable to execute git process")]
    Execution,
    #[error("Unable to decode error from git executable")]
    Undecodable,
    #[error("git URL is invalid")]
    InvalidUrl,
    #[error("git failed with the following message: {0}")]
    GitError(String),
}
