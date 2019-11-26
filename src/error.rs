use std::error::Error;
use std::fmt::{self, Display, Formatter};

//FIXME consider using lib for this
#[derive(Debug)]
pub struct GitError {
    pub message: String,
}

impl Error for GitError {}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
