#![allow(unused)]

use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
    IO,
    FILE,
    TARGET,
}

pub struct Error {
    error_type: ErrorType,
    message: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} error : {}", &self.error_type, &self.message)
    }
}

pub struct ErrorBuilder {
    error_type: Option<ErrorType>,
    message: Option<String>,
}

impl ErrorBuilder {
    // new error builder instance
    pub fn new() -> Self {
        Self {
            error_type: None,
            message: None,
        }
    }
    // adding error type
    pub fn add_error_type(&mut self, error_type: ErrorType) {
        self.error_type = Some(error_type);
    }
    // adding message
    pub fn add_message(&mut self, message: String) {
        self.message = Some(message);
    }
    // building into a Error instance
    pub fn build(&mut self) -> Error {
        Error::new(
            *self.error_type.clone().as_ref().unwrap(),
            self.message.clone().unwrap(),
        )
    }
}
