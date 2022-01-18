use std::{future::Future, process::Output, pin::Pin};

use crate::{data::{User, Thread, Comment}, error::{StoreError, Error}};

pub trait Frontend {
    fn run(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

pub trait UserStore {
    fn save_user(& mut self, user: User) -> Box<dyn Future<Output = Result<User, StoreError>> + Unpin>;
    fn delete_user(& mut self, name: &str) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin>;
    fn find_user(&self, name: &str) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin>;
    fn find_all_users(&self) -> Pin<Box<dyn Future<Output= Result<Vec<User>, StoreError>> + Send + Sync>>;
}

pub trait ThreadStore {
    fn save_thread(& mut self, thread: Thread) -> Box<dyn Future<Output = Result<Thread, StoreError>> + Unpin>;
    fn delete_thread(& mut self, hash: &str) -> Box<dyn Future<Output = Result<Option<Thread>, StoreError>> + Unpin>;
    fn find_thread_by_hash(&self, hash: &str) -> Box<dyn Future<Output = Result<Option<Thread>, StoreError>> + Unpin>;
    fn find_all_threads(&self) -> Box<dyn Future<Output = Result<Vec<Thread>, StoreError>> + Unpin>;
    
}

pub trait CommentStore {
    fn save_comment(& mut self, comment: Comment) -> Box<dyn Future<Output = Result<Comment, StoreError>> + Unpin>;
    fn delete_comment(& mut self, hash: &str) -> Box<dyn Future<Output = Result<Option<Comment>, StoreError>> + Unpin>;
    fn find_all_comments(&self, thread_hash: &str) -> Box<dyn Future<Output = Result<Vec<Comment>, StoreError>> + Unpin>;
}