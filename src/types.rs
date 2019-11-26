use super::GitError;
use regex::Regex;
use std::str::FromStr;

pub struct GitUrl {
    pub(crate) value: String,
}

impl FromStr for GitUrl {
    type Err = GitError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
