use super::GitError;
use regex::Regex;
use std::str::FromStr;
use std::result::Result as stdResult;

pub type Result<A> = stdResult<A, GitError>;

pub struct GitUrl {
    pub(crate) value: String,
}

impl FromStr for GitUrl {
    type Err = GitError;

    fn from_str(value: &str) -> Result<Self> {
        //Regex from https://github.com/jonschlinkert/is-git-url
        let re =
            Regex::new("(?:git|ssh|https?|git@[-\\w.]+):(//)?(.*?)(\\.git)(/?|\\#[-\\d\\w._]+?)$")
                .unwrap();
        if re.is_match(value) {
            Ok(GitUrl {
                value: String::from(value),
            })
        } else {
            Err(GitError::InvalidUrl)
        }
    }
}

pub struct BranchName {
    pub(crate) value: String
}

impl FromStr for BranchName {
    type Err = GitError;
    fn from_str(s: &str) -> Result<Self> { 
        if is_valid_reference_name(s) {
            Ok(BranchName {
                value: String::from(s)
            })
        } else {
            Err(GitError::InvalidRefName)
        }
    }
    
}

const INVALID_REFERENCE_CHARS: [char; 5] = [' ', '~', '^', ':', '\\'];
const INVALID_REFERENCE_START: &str = "-";
const INVALID_REFERENCE_END: &str = ".";

//FIXME test
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_git_urls() {

        let valid_urls = vec!(
            "git://github.com/ember-cli/ember-cli.git#ff786f9f",
            "git://github.com/ember-cli/ember-cli.git#gh-pages",
            "git://github.com/ember-cli/ember-cli.git#master",
            "git://github.com/ember-cli/ember-cli.git#Quick-Fix",
            "git://github.com/ember-cli/ember-cli.git#quick_fix",
            "git://github.com/ember-cli/ember-cli.git#v0.1.0",
            "git://host.xz/path/to/repo.git/",
            "git://host.xz/~user/path/to/repo.git/",
            "git@192.168.101.127:user/project.git",
            "git@github.com:user/project.git",
            "git@github.com:user/some-project.git",
            "git@github.com:user/some-project.git",
            "git@github.com:user/some_project.git",
            "git@github.com:user/some_project.git",
            "http://192.168.101.127/user/project.git",
            "http://github.com/user/project.git",
            "http://host.xz/path/to/repo.git/",
            "https://192.168.101.127/user/project.git",
            "https://github.com/user/project.git",
            "https://host.xz/path/to/repo.git/",
            "https://username::;*%$:@github.com/username/repository.git",
            "https://username:$fooABC@:@github.com/username/repository.git",
            "https://username:password@github.com/username/repository.git",
            "ssh://host.xz/path/to/repo.git/",
            "ssh://host.xz/path/to/repo.git/",
            "ssh://host.xz/~/path/to/repo.git",
            "ssh://host.xz/~user/path/to/repo.git/",
            "ssh://host.xz:port/path/to/repo.git/",
            "ssh://user@host.xz/path/to/repo.git/",
            "ssh://user@host.xz/path/to/repo.git/",
            "ssh://user@host.xz/~/path/to/repo.git",
            "ssh://user@host.xz/~user/path/to/repo.git/",
            "ssh://user@host.xz:port/path/to/repo.git/",
        );

        for url in valid_urls.iter() {  
            assert!(GitUrl::from_str(url).is_ok())
        }
    }


    #[test]
    fn test_invalid_git_urls() {
        let invalid_urls = vec!(
            "/path/to/repo.git/",
            "file:///path/to/repo.git/",
            "file://~/path/to/repo.git/",
            "git@github.com:user/some_project.git/foo",
            "git@github.com:user/some_project.gitfoo",
            "host.xz:/path/to/repo.git/",
            "host.xz:path/to/repo.git",
            "host.xz:~user/path/to/repo.git/",
            "path/to/repo.git/",
            "rsync://host.xz/path/to/repo.git/",
            "user@host.xz:/path/to/repo.git/",
            "user@host.xz:path/to/repo.git",
            "user@host.xz:~user/path/to/repo.git/",
            "~/path/to/repo.git"
        );

        for url in invalid_urls.iter() {  
            assert!(GitUrl::from_str(url).is_err())
        }
    }
}