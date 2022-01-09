use std::{
    future::Future,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
};

use comments_rs_core::{
    data::{Comment, Thread, User},
    error::{Error, StoreError},
    traits::{CommentStore, UserStore, ThreadStore},
};

#[derive(Default)]
struct CommentDb {
    users: Vec<User>,
    threads: Vec<Thread>,
    comments: Vec<Comment>,
}

#[derive(Default)]
struct DbWrapper {
    data: Box<Mutex<CommentDb>>
}

struct ImmediateFuture<T> {
    result: Result<T, StoreError>,
}

impl<T> Future for ImmediateFuture<T>
where
    T: Clone,
{
    type Output = Result<T, StoreError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.result.clone())
    }
}

impl UserStore for DbWrapper {
    fn save_user(&mut self, user: User) -> Box<dyn Future<Output = Result<User, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();
        data.users.push(user);

        Box::new(ImmediateFuture {
            result: Ok(data.users.last().unwrap().clone()),
        })
    }

    fn find_user(
        &self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin> {
        let data = self.data.lock().unwrap();
        let option_user = data.users
            .iter()
            .find(|user| user.name.as_str() == name)
            .map(|user| user.clone());

        Box::new(ImmediateFuture {
            result: Ok(option_user),
        })
    }

    fn delete_user(
        &mut self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<Option<User>, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();

        let index = data.users.iter().position(|user| user.name.as_str() == name);

        match index {
            Some(index) => Box::new(ImmediateFuture {
                result: Ok(Some(data.users.remove(index))),
            }),
            None => Box::new(ImmediateFuture { result: Ok(None) }),
        }
    }
}

impl ThreadStore for DbWrapper {

    fn save_thread(
        &mut self,
        thread: Thread,
    ) -> Box<dyn Future<Output = Result<Thread, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();
        data.threads.push(thread);

        Box::new(ImmediateFuture {
            result: Ok(data.threads.last().unwrap().clone()),
        })
    }


    fn delete_thread(
        &mut self,
        hash: &str,
    ) -> Box<dyn Future<Output = Result<Option<Thread>, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();

        let index = data.threads.iter().position(|thread| thread.hash == hash);

        match index {
            Some(index) => Box::new(ImmediateFuture {
                result: Ok(Some(data.threads.remove(index))),
            }),
            None => Box::new(ImmediateFuture { result: Ok(None) }),
        }
    }

    fn find_thread_by_hash(
        &self,
        hash: &str,
    ) -> Box<dyn Future<Output = Result<Option<Thread>, StoreError>> + Unpin> {
        todo!()
    }

    fn find_all_threads(
        &self,
    ) -> Box<dyn Future<Output = Result<Vec<Thread>, StoreError>> + Unpin> {
        todo!()
    }
}

impl CommentStore for DbWrapper {
    
    fn save_comment(
        &mut self,
        comment: Comment,
    ) -> Box<dyn Future<Output = Result<Comment, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();

        let thread = data.threads
            .iter()
            .find(|thread| thread.hash == comment.thread_hash);

        match thread {
            Some(_) => {
                data.comments.push(comment);

                Box::new(ImmediateFuture {
                    result: Ok(data.comments.last().unwrap().clone()),
                })
            }
            None => Box::new(ImmediateFuture {
                result: Err(StoreError::ThreadNotExists(comment.thread_hash.into())),
            }),
        }
    }

    fn delete_comment(
        &mut self,
        hash: &str,
    ) -> Box<dyn Future<Output = Result<Option<Comment>, StoreError>> + Unpin> {
        todo!()
    }

    fn find_all_comments(
        &self,
        thread_hash: &str,
    ) -> Box<dyn Future<Output = Result<Vec<Comment>, StoreError>> + Unpin> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use comments_rs_core::{
        data::{Thread, User, Comment},
        traits::{CommentStore, UserStore, ThreadStore},
    };

    use crate::{CommentDb, DbWrapper};

    #[tokio::test]
    async fn test_save_user() {
        let mut user_db = DbWrapper::default();
        let user = User::new("test@mail.com", "name");

        let saved_user = user_db.save_user(user).await.unwrap();

        assert_eq!(saved_user.name, "name");
        assert_eq!(saved_user.email, "test@mail.com");
    }

    #[tokio::test]
    async fn test_find_user() {
        let mut user_db = DbWrapper::default();
        let user = User::new("test@mail.com", "name");
        let user1 = User::new("test@mail.com", "name1");

        user_db.save_user(user.clone()).await.unwrap();

        assert_eq!(user_db.find_user("name").await.unwrap(), Some(user));
        assert_eq!(user_db.find_user("name1").await.unwrap(), None);

        user_db.save_user(user1.clone()).await.unwrap();
        assert_eq!(user_db.find_user("name1").await.unwrap(), Some(user1));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut user_db = DbWrapper::default();
        let user = User::new("test@mail.com", "name");
        let user1 = User::new("test@mail.com", "name1");

        user_db.save_user(user.clone()).await.unwrap();
        user_db.save_user(user1.clone()).await.unwrap();

        assert_eq!(user_db.find_user("name1").await.unwrap(), Some(user1.clone()));
        assert_eq!(user_db.delete_user("name1").await.unwrap(), Some(user1));
        assert_eq!(user_db.delete_user("name1").await.unwrap(), None);
        assert_eq!(user_db.find_user("name1").await.unwrap(), None)
    }

    #[tokio::test]
    async fn test_save_thread() {
        let mut comment_db = DbWrapper::default();
        let thread = Thread::new("thread");

        let save_result = comment_db.save_thread(thread.clone()).await.unwrap();

        assert_eq!(save_result, thread);
    }

    #[tokio::test]
    async fn test_delete_thread() {
        let mut comment_db = DbWrapper::default();
        let thread = Thread::new("thread");

        let save_result = comment_db.save_thread(thread.clone()).await.unwrap();

        assert_eq!(save_result, thread.clone());

        let deleted_thread = comment_db
            .delete_thread(thread.hash.as_str())
            .await
            .unwrap();

        assert_eq!(deleted_thread, Some(thread.clone()));

        let deleted_thread = comment_db
            .delete_thread(thread.hash.as_str())
            .await
            .unwrap();

        assert_eq!(deleted_thread, None);
    }

    #[tokio::test]
    async fn test_save_comment() {
        let mut comment_db = DbWrapper::default();
        let thread = Thread::new("thread");

        let save_result = comment_db.save_thread(thread.clone()).await.unwrap();

        assert_eq!(save_result, thread);

        let comment = Comment::new(thread.hash.as_str(), "user@mail.com", 17, "content");
    }
}
