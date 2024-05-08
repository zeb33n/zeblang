use std::io::{Error, ErrorKind};

pub fn new_error(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidInput, msg)
}
