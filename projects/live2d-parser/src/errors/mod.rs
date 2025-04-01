use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum L2Error {
    OutOfBounds { rest: usize, request: usize },
    Error {},
}

impl Error for L2Error {}

impl Display for L2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            L2Error::OutOfBounds { rest, request } => {
                f.write_str(&format!("Out of bounds: rest={}, request={}", rest, request))
            }
            L2Error::Error {} => f.write_str(""),
        }
    }
}
