use std::future::Future;

use crate::{data::User, error::{Error, StoreError}};

pub trait UserStore {
    fn save(& mut self, user: User) -> Box<dyn Future<Output = Result<User, StoreError>> + Unpin>;
    fn delete(& mut self, name: &str) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin>;
    fn find(&self, name: &str) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin>;
}