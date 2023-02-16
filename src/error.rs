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
    #[error("Ref name is invalid")]
    InvalidRefName,
    #[error("git failed with the following stdout: {stdout} stderr: {stderr}")]
    GitError{
        stdout: String,
        stderr: String
    },
    #[error("No Git Repository is available")]
    NoRemoteRepositorySet
}
