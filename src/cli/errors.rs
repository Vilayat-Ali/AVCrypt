#![allow(unused)]

use clap::error::{Error, ErrorFormatter, ErrorKind};
use std::fmt::{self, Formatter};

pub struct ErrorBuilder {
    error_type: Option<ErrorKind>,
    error_message: Option<String>,
}

impl ErrorBuilder {
    pub fn new() -> Self {
        Self {
            error_type: None,
            error_message: None,
        }
    }

    pub fn add_error_type(&mut self, error_type: ErrorKind) -> &mut ErrorBuilder {
        self.error_type = Some(error_type);
        self
    }

    pub fn add_error_message(&mut self, error_message: String) -> &mut ErrorBuilder {
        self.error_message = Some(error_message);
        self
    }

    pub fn build(&mut self) -> Error {
        Error::new(self.error_type.unwrap())
    }
}
