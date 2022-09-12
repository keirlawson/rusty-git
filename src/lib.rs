use error::GitError;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use types::{BranchName, GitUrl, Result};

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
        execute_git(
            &self.location,
            &["checkout", "-b", branch_name.value.as_str()],
        )
    }

    ///Checkout the specified branch
    pub fn switch_branch(&self, branch_name: &BranchName) -> Result<()> {
        execute_git(&self.location, &["checkout", branch_name.value.as_str()])
    }

    ///Add file contents to the index
    pub fn add(&self, pathspecs: Vec<&str>) -> Result<()> {
        let mut args = pathspecs.clone();
        args.insert(0, "add");
        execute_git(&self.location, args)
    }

    ///Remove file contents from the index
    pub fn remove(&self, pathspecs: Vec<&str>, force: bool) -> Result<()> {
        let mut args = pathspecs.clone();
        args.insert(0, "rm");
        if force {
            args.push("-f");
        }
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

    ///Push the curent branch to its associated remote, specifying the upstream branch
    pub fn push_to_upstream(&self, upstream: &str, upstream_branch: &BranchName) -> Result<()> {
        execute_git(
            &self.location,
            &["push", "-u", upstream, upstream_branch.value.as_str()],
        )
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
    pub fn create_branch_from_startpoint(
        &self,
        branch_name: &BranchName,
        startpoint: &str,
    ) -> Result<()> {
        execute_git(
            &self.location,
            &[
                "checkout",
                "-b",
                branch_name.to_string().as_str(),
                startpoint,
            ],
        )
    }

    ///List local branches
    pub fn list_branches(&self) -> Result<Vec<String>> {
        execute_git_fn(
            &self.location,
            &["branch", "--format=%(refname:short)"],
            |output| output.lines().map(|line| line.to_owned()).collect(),
        )
    }

    ///List files added to staging area
    pub fn list_added(&self) -> Result<Vec<String>> {
        git_status(&self, "A")
    }

    ///List all modified files
    pub fn list_modified(&self) -> Result<Vec<String>> {
        git_status(&self, " M")
    }

    ///List all untracked files
    pub fn list_untracked(&self) -> Result<Vec<String>> {
        git_status(&self, "??")
    }

    ///List tracked files
    pub fn list_tracked(&self) -> Result<Vec<String>> {
        execute_git_fn(&self.location, &["ls-files"], |output| {
            output.lines().map(|line| line.to_owned()).collect()
        })
    }

    /// Obtains commit hash of the current `HEAD`.
    pub fn get_hash(&self, short: bool) -> Result<String> {
        let args: &[_] = if short {
            &["rev-parse", "--short", "HEAD"]
        } else {
            &["rev-parse", "HEAD"]
        };
        execute_git_fn(&self.location, args, |output| output.trim().to_owned())
    }

    /// Execute user defined command
    pub fn cmd<I, S>(&self, args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        execute_git(&self.location, args)
    }

    /// Execute user defined command and return its output
    pub fn cmd_out<I, S>(&self, args: I) -> Result<Vec<String>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        execute_git_fn(&self.location, args, |output| {
            output.lines().map(|line| line.to_owned()).collect()
        })
    }
}

fn git_status(repo: &Repository, prefix: &str) -> Result<Vec<String>> {
    execute_git_fn(&repo.location, &["status", "-s"], |output| {
        output
            .lines()
            .filter(|line| line.starts_with(&prefix))
            .map(|line| line[3..].to_owned())
            .collect()
    })
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
    F: Fn(&str) -> R,
{
    let output = Command::new("git").current_dir(p).args(args).output();

    output.map_err(|_| GitError::Execution).and_then(|output| {
        if output.status.success() {
            if let Ok(message) = str::from_utf8(&output.stdout) {
                Ok(process(message))
            } else {
                Err(GitError::Undecodable)
            }
        } else {
            if let Ok(stdout) = str::from_utf8(&output.stdout) {
                if let Ok(stderr) = str::from_utf8(&output.stderr) {
                    Err(GitError::GitError {
                        stdout: stdout.to_owned(),
                        stderr: stderr.to_owned(),
                    })
                } else {
                    Err(GitError::Undecodable)
                }
            } else {
                Err(GitError::Undecodable)
            }
        }
    })
}
