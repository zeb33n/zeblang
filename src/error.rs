use std::io::{Error, ErrorKind};

pub fn new_error(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidInput, msg)
}

pub fn syntax_error(msg: &str, line: usize) -> Error {
    Error::new(
        ErrorKind::InvalidInput,
        &*format!("{}: {}", line, msg).as_str(),
    )
}
