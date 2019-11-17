use error::GitError;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use std::env;

mod error;

pub struct GitUrl {
    value: String,
}

impl GitUrl {
    //FIXME do validation here
    pub const fn new(value: String) -> GitUrl {
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

        let cwd = env::current_dir().map_err(|_| GitError {
            message: String::from("Unable to access current working directory")
        })?;
        execute_git(cwd, &["clone", url.value.as_str(), p.to_str().unwrap()]).map(|_| Repository {
            location: PathBuf::from(p),
        })
    }

    //Create and checkout a new local branch
    pub fn create_local_branch(&self, branch_name: &str) -> Result<(), GitError> {
        execute_git(&self.location, &["checkout", "-b", branch_name])
    }

    //Checkout the specified branch
    pub fn switch_branch(&self, branch_name: &str) -> Result<(), GitError> {
        execute_git(&self.location, &["checkout", branch_name])
    }

    //Commit all staged files
    pub fn commit_all(&self) -> Result<(), GitError> {
        execute_git(&self.location, &["commit", "-a"])
    }
}

fn execute_git<I, S, P>(p: P, args: I) -> Result<(), GitError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
        P: AsRef<Path>
    {
        let output = Command::new("git")
            .current_dir(p)
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