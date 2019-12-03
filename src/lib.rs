use error::GitError;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use types::{GitUrl, Result};

mod error;
pub mod types;

const INVALID_REFERENCE_CHARS: [char; 5] = [' ', '~', '^', ':', '\\'];
const INVALID_REFERENCE_START: &str = "-";
const INVALID_REFERENCE_END: &str = ".";

fn is_valid_reference_name(name: &str) -> bool {
    !name.starts_with(INVALID_REFERENCE_START)
        && !name.ends_with(INVALID_REFERENCE_END)
        && name.chars().all(|c| {
            !c.is_ascii_control() && INVALID_REFERENCE_CHARS.iter().all(|invalid| &c != invalid)
        })
        && !name.contains("/.")
        && !name.contains("@{")
        && !name.contains("..")
        && name != "@"
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
    pub fn clone<P: AsRef<Path>>(url: GitUrl, p: P) -> Result<Repository> {
        let p = p.as_ref();

        let cwd = env::current_dir().map_err(|_| GitError::WorkingDirectoryInaccessible)?;
        execute_git(cwd, &["clone", url.value.as_str(), p.to_str().unwrap()]).map(|_| Repository {
            location: PathBuf::from(p),
        })
    }

    ///Create and checkout a new local branch
    pub fn create_local_branch(&self, branch_name: &str) -> Result<()> {
        execute_git(&self.location, &["checkout", "-b", branch_name])
    }

    ///Checkout the specified branch
    pub fn switch_branch(&self, branch_name: &str) -> Result<()> {
        execute_git(&self.location, &["checkout", branch_name])
    }

    ///Commit all staged files
    pub fn commit_all(&self) -> Result<()> {
        execute_git(&self.location, &["commit", "-a"])
    }

    ///Push the curent branch to its associated remote
    pub fn push(&self) -> Result<()> {
        execute_git(&self.location, &["push"])
    }

    ///Add a new remote
    pub fn add_remote(&self, name: &str, url: &GitUrl) -> Result<()> {
        execute_git(&self.location, &["remote", "add", name, url.value.as_str()])
    }

    ///Fetch a remote
    pub fn fetch_remote(&self, remote: &str) -> Result<()> {
        execute_git(&self.location, &["fetch", remote])
    }

    ///Create a new branch from a start point, such as another local or remote branch
    pub fn create_branch_from_startpoint(&self, branch_name: &str, startpoint: &str) -> Result<()> {
        execute_git(&self.location, &["checkout", "-b", branch_name, startpoint])
    }

}

fn execute_git<I, S, P>(p: P, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let output = Command::new("git").current_dir(p).args(args).output();

    output.map_err(|_| GitError::Execution).and_then(|output| {
        if output.status.success() {
            Ok(())
        } else if let Ok(message) = str::from_utf8(&output.stderr) {
            Err(GitError::GitError(message.to_owned()))
        } else {
            Err(GitError::Undecodable)
        }
    })
}
