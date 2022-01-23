use std::fmt::Display;

use validator::ValidationErrors;

pub trait CommentError<'a>: std::error::Error {
    fn code(&'a self) -> &'a str;
    fn inner(&'a self) -> &'a Error;
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ValidationError{ validation_errors: ValidationErrors },
    StoreError(StoreError),
    SignupError,
    NewtorkError,
    SignatureError(Vec<String>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StoreError {
    NameNotUnique,
    ThreadNotExists(String)
}

impl<'a> CommentError<'a> for Error {

    fn code(&'a self) -> &'a str {
        match self {
            Error::ValidationError { validation_errors: _ } => "E-00-01",
            Error::StoreError(store_error) => {
                match store_error {
                    StoreError::NameNotUnique => "E-01-01",
                    StoreError::ThreadNotExists(_) => "E-01-02",
                }
            },
            Error::NewtorkError => todo!(),
            Error::SignupError => todo!(),
            Error::SignatureError(_) => todo!(),
        }
    }

    fn inner(&'a self) -> &'a Error {
        self
    }
}

struct PrettyReport {
    message: String
}

impl Display for PrettyReport {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<&Error> for PrettyReport {
    fn from(_: &Error) -> Self {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Into::<PrettyReport>::into(self).fmt(f)
    }
}

impl std::error::Error for Error {}