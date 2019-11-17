use error::GitError;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

mod error;

pub struct GitUrl {
    value: String,
}

impl GitUrl {
    pub fn new(value: String) -> GitUrl {
        GitUrl { value }
    }
}

pub struct Repository {
    location: PathBuf,
}

impl Repository {
    ///Create a Repository struct from a pre-existing local git repository
    pub fn new<P: AsRef<Path>>(p: P) -> Repository {
        let p = p.as_ref();
        Repository {
            location: PathBuf::from(p),
        }
    }

    ///Clone a remote git repository locally
    pub fn clone<P: AsRef<Path>>(url: GitUrl, p: P) -> Result<Repository, GitError> {
        let p = p.as_ref();

        Command::new("git")
            .arg("clone")
            .arg(url.value)
            .arg(p)
            .output()
            .map_err(|_| GitError {
                message: String::from("unable to execute git process"),
            })
            .and_then(|output| {
                if output.status.success() {
                    Ok(Repository {
                        location: PathBuf::from(p),
                    })
                } else {
                    let message =
                        str::from_utf8(&output.stderr).unwrap_or_else(|_| "unable to decode error");
                    Err(GitError {
                        message: String::from(message),
                    })
                }
            })
    }

    //Create and checkout a new local branch
    pub fn create_local_branch(&self, branch_name: &str) -> Result<(), GitError> {
        self.execute_git(&["checkout", "-b", branch_name])
    }

    //Checkout the specified branch
    pub fn switch_branch(&self, branch_name: &str) -> Result<(), GitError> {
        self.execute_git(&["checkout", branch_name])
    }

    //Commit all staged files
    pub fn commit_all(&self) -> Result<(), GitError> {
        self.execute_git(&["commit", "-a"])
    }

    fn execute_git<I, S>(&self, args: I) -> Result<(), GitError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = Command::new("git")
            .current_dir(&self.location)
            .args(args)
            .output();

        output
            .map_err(|_| GitError {
                message: String::from("unable to execute git process"),
            })
            .and_then(|output| {
                if output.status.success() {
                    Ok(())
                } else {
                    let message =
                        str::from_utf8(&output.stderr).unwrap_or_else(|_| "unable to decode error");
                    Err(GitError {
                        message: String::from(message),
                    })
                }
            })
    }
}
