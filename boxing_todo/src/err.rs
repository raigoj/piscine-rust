use std::fmt;
use std::fmt::{Debug, Display};
use std::error::Error;
#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>)
}
// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to parses todo")
    }
}
#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}
// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to read todo file")
    }
}
impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(_) => Some(self),
        }
    }
}
impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.child_err)
    }
}