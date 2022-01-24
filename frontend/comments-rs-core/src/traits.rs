use std::{pin::Pin, future::Future};

use crate::{structs::{Thread, Comment}, error::Error};

pub type StoreResult<T> = Pin<Box<dyn Future<Output = Result<Option<T>, Error>>>>;

pub trait ThreadStore {
    fn load(&self, hash: &str) -> StoreResult<Thread>;
}

pub trait CommentStore {
    fn load(&self, hash: &str, offset: i64, limit: i64) -> StoreResult<Page<Comment>>;
}

pub struct Page<T> {
    pub page_no: i64,
    pub page_count: i64,
    pub elements: Vec<T>
}