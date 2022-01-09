use std::{fmt::Display, io::Read};

use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::error::{CommentError, Error};

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"[a-zA-Z0-9 \.\$—]{4,32}$").unwrap();
    static ref CONTENT_REGEX: Regex = Regex::new(r".{1,2048}$").unwrap();
}

pub struct Thread {
    name: String,
    hash: String
}

#[derive(Validate, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    #[validate(regex = "NAME_REGEX")]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Validate, Clone, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub thread: String,
    pub date: u128,
    #[validate(regex = "CONTENT_REGEX")]
    pub content: String,
}

impl User {
    pub fn new(name: &str, email: &str) -> Self {
        User::try_new(name, email).expect("Invalid user!")
    }

    pub fn try_new(name: &str, email: &str) -> Result<Self, Error> {
        let user = User {
            name: name.into(),
            email: email.into(),
        };

        Ok(user.validate().map(|_| user)?)
    }
}

impl Comment {
    pub fn new(thread: String, date: u128, content: &str) -> Self {
        Comment::try_new(thread, date, content).expect("Invalid comment!")
    }

    pub fn try_new(thread: String, date: u128, content: &str) -> Result<Self, Error> {
        let comment = Comment {
            thread,
            date,
            content: content.into(),
        };

        Ok(comment.validate().map(|_| comment)?)
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Error::ValidationError {
            validation_errors: e,
        }
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[cfg(test)]
mod tests {
    use std::{time::{SystemTime, UNIX_EPOCH}, hash::Hasher};

    use super::{Comment, User};

    #[test]
    fn test_validate_user() {
        let user = User::try_new("name", "email");
        assert!(user.is_err());

        let user = User::try_new("name", "test@mail.de");
        assert!(user.is_ok());
    }

    #[test]
    fn test_validate_comment() {
        let comment = Comment::try_new(
            DefaultHasher::new().hash("").finish(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            "test\nhaving special chars: /$.",
        );

        assert!(comment.is_ok());
    }
}
