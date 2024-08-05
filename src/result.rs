use std::{io, result};
use crate::result::Error::*;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    STRING(String)
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        IO(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        STRING(value)
    }
}