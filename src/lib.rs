use std::path::{Path, PathBuf};
use std::process::Command;

mod error;

pub struct GitUrl {
    value: String
}

impl GitUrl {
    pub fn new(value: String) -> GitUrl {
        GitUrl {
            value
        }
    }
}

pub struct Repository {
    location: PathBuf
}

impl Repository {
    ///Create a Repository struct from a pre-existing local git repository
    pub fn new<P: AsRef<Path>>(p: P) -> Repository {
        let p = p.as_ref();
        Repository {
            location: PathBuf::from(p)
        }
    }

    ///Clone a remote git repository locally
    pub fn clone<P: AsRef<Path>>(url: GitUrl, p: P) -> Result<Repository, error::GitError> {
        let p = p.as_ref();

        Command::new("git")
            .arg("clone")
            .arg(url.value)
            .arg(p)
            .output()
            .expect("failed to execute process");

        Ok(Repository {
            location: PathBuf::from(p)
        })
    }
}