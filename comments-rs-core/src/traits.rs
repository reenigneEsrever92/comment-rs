use std::{future::Future, pin::Pin};

use crate::{data::{User, Thread, Comment}, error::{StoreError, Error}};

pub trait Frontend {
    fn run(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

pub type StoreResult<T> = Pin<Box<dyn Future<Output = Result<T, StoreError>> + Send + Sync>>;

pub trait UserStore: Send + Sync {
    fn save_user(& mut self, user: User) -> StoreResult<User>;
    fn delete_user(& mut self, name: &str) -> StoreResult<Option<User>>;
    fn find_user(&self, name: &str) -> StoreResult<Option<User>>;
    fn find_all_users(&self) -> StoreResult<Vec<User>>;
}

pub trait ThreadStore: Send + Sync {
    fn save_thread(& mut self, thread: Thread) -> StoreResult<Thread>;
    fn delete_thread(& mut self, hash: &str) -> StoreResult<Option<Thread>>;
    fn find_thread_by_hash(&self, hash: &str) -> StoreResult<Option<Thread>>;
    fn find_all_threads(&self) -> StoreResult<Vec<Thread>>;
    
}

pub trait CommentStore: Send + Sync {
    fn save_comment(& mut self, comment: Comment) -> StoreResult<Comment>;
    fn delete_comment(& mut self, hash: &str) -> StoreResult<Option<Comment>>;
    fn find_thread_comments(&self, thread_hash: &str) -> StoreResult<Vec<Comment>>;
}