use std::{
    future::Future,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
};

use comments_rs_core::{
    data::User,
    error::{Error, StoreError},
    traits::UserStore,
};

#[derive(Default)]
struct UserDb {
    data: Box<Mutex<Vec<User>>>,
}

struct ImmediateFuture<T> {
    result: T,
}

impl Future for ImmediateFuture<User> {
    type Output = Result<User, StoreError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(self.result.clone()))
    }
}

impl Future for ImmediateFuture<Option<User>> {
    type Output = Result<Option<User>, StoreError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(self.result.clone()))
    }
}

impl UserStore for UserDb {
    fn save(&mut self, user: User) -> Box<dyn Future<Output = Result<User, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();
        data.push(user);

        Box::new(ImmediateFuture {
            result: data.last().unwrap().clone(),
        })
    }

    fn find(
        &self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin> {
        let data = self.data.lock().unwrap();
        let option_user = data
            .iter()
            .find(|user| user.name.as_str() == name)
            .map(|user| user.clone());

        Box::new(ImmediateFuture {
            result: option_user,
        })
    }

    fn delete(
        &mut self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();

        let index = data.iter().position(|user| user.name.as_str() == name);

        match index {
            Some(index) => Box::new(ImmediateFuture {
                result: Some(data.remove(index)),
            }),
            None => Box::new(ImmediateFuture { result: None }),
        }
    }
}

#[cfg(test)]
mod tests {
    use comments_rs_core::{data::User, traits::UserStore};

    use crate::UserDb;

    #[tokio::test]
    async fn test_save_user() {
        let mut user_db = UserDb::default();
        let user = User::new("name", "test@mail.com");

        let saved_user = user_db.save(user).await.unwrap();

        assert_eq!(saved_user.name, "name");
        assert_eq!(saved_user.email, "test@mail.com");
    }

    #[tokio::test]
    async fn test_find_user() {
        let mut user_db = UserDb::default();
        let user = User::new("name", "test@mail.com");
        let user1 = User::new("name1", "test@mail.com");

        user_db.save(user.clone()).await.unwrap();

        assert_eq!(user_db.find("name").await.unwrap(), Some(user));
        assert_eq!(user_db.find("name1").await.unwrap(), None);

        user_db.save(user1.clone()).await.unwrap();
        assert_eq!(user_db.find("name1").await.unwrap(), Some(user1));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut user_db = UserDb::default();
        let user = User::new("name", "test@mail.com");
        let user1 = User::new("name1", "test@mail.com");

        user_db.save(user.clone()).await.unwrap();
        user_db.save(user1.clone()).await.unwrap();

        assert_eq!(user_db.find("name1").await.unwrap(), Some(user1.clone()));
        assert_eq!(user_db.delete("name1").await.unwrap(), Some(user1));
        assert_eq!(user_db.find("name1").await.unwrap(), None)
    }
}
