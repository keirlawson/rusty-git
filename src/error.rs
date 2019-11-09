use std::fmt::{self, Display, Formatter};
use std::error::Error;

#[derive(Debug)]
struct GitError {
    message: String
}

impl Error for GitError {

}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}