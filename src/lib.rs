use error::GitError;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use types::{GitUrl, Result, BranchName};

pub mod error;
pub mod types;

///A local git repository
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

    ///Initialise a given folder as a git repository
    pub fn init<P: AsRef<Path>>(p: P) -> Result<Repository> {
        let p = p.as_ref();
        execute_git(&p, &["init"])?;
        Ok(Repository {
            location: PathBuf::from(p),
        })
    }


    ///Create and checkout a new local branch
    pub fn create_local_branch(&self, branch_name: &BranchName) -> Result<()> {
        execute_git(&self.location, &["checkout", "-b", branch_name.value.as_str()])
    }

    ///Checkout the specified branch
    pub fn switch_branch(&self, branch_name: &BranchName) -> Result<()> {
        execute_git(&self.location, &["checkout", branch_name.value.as_str()])
    }

    ///Add file contents to the index
    pub fn add(&self, pathspecs: Vec<&str>) -> Result<()> 
    {
        let mut args = pathspecs.clone();
        args.insert(0, "add");
        execute_git(&self.location, args)
    }

    ///Commit all staged files
    pub fn commit_all(&self, message: &str) -> Result<()> {
        execute_git(&self.location, &["commit", "-am", message])
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

    ///List local branches
    pub fn list_branches(&self) -> Result<Vec<String>> {
        execute_git_fn(&self.location, &["branch", "--format=%(refname:short)"], |output| {
            output.lines().map(|line| line.to_owned()).collect()
        })
    }

}

fn execute_git<I, S, P>(p: P, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    execute_git_fn(p, args, |_| ())
}


fn execute_git_fn<I, S, P, F, R>(p: P, args: I, process: F) -> Result<R>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
    F: Fn(&str) -> R
{
    let output = Command::new("git").current_dir(p).args(args).output();

    output.map_err(|_| GitError::Execution).and_then(|output| {
        if output.status.success() {
            if let Ok(message) = str::from_utf8(&output.stdout) {
                Ok(process(message))
            } else {
                Err(GitError::Undecodable)
            }
        } else if let Ok(message) = str::from_utf8(&output.stderr) {
            dbg!(&output);
            Err(GitError::GitError(message.to_owned()))
        } else {
            Err(GitError::Undecodable)
        }
    })
}
