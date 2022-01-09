use std::str;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use validator::{Validate, ValidationErrors};

use crate::error::Error;

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"[a-zA-Z0-9 \.\$—]{4,32}$").unwrap();
    static ref CONTENT_REGEX: Regex = Regex::new(r".{1,2048}$").unwrap();
    static ref HASH_REGEX: Regex = Regex::new(r"[0-9a-f]{64}$").unwrap();
}

#[derive(Validate, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Thread {
    pub name: String,
    #[validate(regex = "HASH_REGEX")]
    pub hash: String,
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
    #[validate(regex = "HASH_REGEX")]
    pub thread_hash: String,
    #[validate(regex = "HASH_REGEX")]
    pub hash: String,
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
    pub fn new(thread_hash: String, date: u128, content: &str) -> Self {
        Comment::try_new(thread_hash, date, content).expect("Invalid comment!")
    }

    pub fn try_new(thread_hash: String, date: u128, content: &str) -> Result<Self, Error> {
        let bytes = [date.to_be_bytes().as_slice(), content.as_bytes()]
            .concat();

        let comment = Comment {
            hash: hash(bytes.as_slice()),
            thread_hash,
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

fn hash(input: &[u8]) -> String {
    hex::encode(sha2::Sha256::digest(input))
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::data::hash;

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
            hash("test".as_bytes()),
            1,
            "test\nhaving special chars: /$.",
        );

        assert!(comment.is_ok());

        let comment = comment.unwrap();

        assert_eq!(
            comment.thread_hash,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".to_string()
        );
        assert_eq!(
            comment.hash,
            "24ebe193b02114aeb806e5f5d1e73ec6b5e9c3d6089d938ebb9a81137b97ad57".to_string()
        );
    }
}
