use std::{
    future::Future,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
};

use comments_rs_core::{
    data::{Comment, Thread, User},
    error::StoreError,
    traits::{CommentStore, ThreadStore, UserStore},
};

#[derive(Default)]
struct CommentDb {
    users: Vec<User>,
    threads: Vec<Thread>,
    comments: Vec<Comment>,
}

#[derive(Default)]
struct DbWrapper {
    data: Box<Mutex<CommentDb>>,
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
    fn save_user(
        &mut self,
        user: User,
    ) -> Box<dyn Future<Output = Result<User, StoreError>> + Unpin> {
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
        let option_user = data
            .users
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

        let index = data
            .users
            .iter()
            .position(|user| user.name.as_str() == name);

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
        let data = self.data.lock().unwrap();

        Box::new(ImmediateFuture {
            result: Ok(data
                .threads
                .iter()
                .find(|thread| thread.hash == hash)
                .map(|thread| thread.clone())),
        })
    }

    fn find_all_threads(
        &self,
    ) -> Box<dyn Future<Output = Result<Vec<Thread>, StoreError>> + Unpin> {
        let data = self.data.lock().unwrap();

        Box::new(ImmediateFuture {
            result: Ok(data.threads.clone()),
        })
    }
}

impl CommentStore for DbWrapper {
    fn save_comment(
        &mut self,
        comment: Comment,
    ) -> Box<dyn Future<Output = Result<Comment, StoreError>> + Unpin> {
        let data = self.data.get_mut().unwrap();

        let thread = data
            .threads
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
        let data = self.data.get_mut().unwrap();

        let index = data
            .comments
            .iter()
            .position(|comment| comment.hash == hash);

        match index {
            Some(index) => Box::new(ImmediateFuture {
                result: Ok(Some(data.comments.remove(index))),
            }),
            None => Box::new(ImmediateFuture { result: Ok(None) }),
        }
    }

    fn find_all_comments(
        &self,
        thread_hash: &str,
    ) -> Box<dyn Future<Output = Result<Vec<Comment>, StoreError>> + Unpin> {
        let data = self.data.lock().unwrap();

        let comments: Vec<Comment> = data
            .comments
            .iter()
            .filter(|comment| comment.thread_hash == thread_hash)
            .map(|comment| comment.clone())
            .collect();

        Box::new(ImmediateFuture {
            result: Ok(comments),
        })
    }
}

#[cfg(test)]
mod tests {
    use comments_rs_core::{
        data::{Comment, Thread, User},
        traits::{CommentStore, ThreadStore, UserStore},
    };

    use crate::DbWrapper;

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

        assert_eq!(
            user_db.find_user("name1").await.unwrap(),
            Some(user1.clone())
        );
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
    async fn test_find_thread_by_hash() {
        let mut comment_db = DbWrapper::default();
        let thread = Thread::new("thread");

        let _saved_thread = comment_db.save_thread(thread.clone()).await.unwrap();

        assert_eq!(
            comment_db
                .find_thread_by_hash(thread.hash.as_str())
                .await
                .unwrap(),
            Some(thread)
        );
    }

    #[tokio::test]
    async fn test_find_all_threads() {
        let mut comment_db = DbWrapper::default();
        let thread = Thread::new("thread");

        let saved_thread = comment_db.save_thread(thread.clone()).await.unwrap();

        assert_eq!(
            comment_db.find_all_threads().await.unwrap(),
            vec![saved_thread.clone()]
        );

        let saved_thread_2 = comment_db.save_thread(thread.clone()).await.unwrap();

        assert_eq!(
            comment_db.find_all_threads().await.unwrap(),
            vec![saved_thread, saved_thread_2]
        );
    }

    #[tokio::test]
    async fn test_save_comment() {
        let mut comment_db = DbWrapper::default();
        let thread = comment_db.save_thread(Thread::new("thread")).await.unwrap();
        let comment = Comment::new(thread.hash.as_str(), "user@mail.com", 17, "content");
        let saved_comment = comment_db.save_comment(comment.clone()).await.unwrap();

        assert_eq!(saved_comment, comment);
    }

    #[tokio::test]
    async fn test_delete_comment() {
        let mut comment_db = DbWrapper::default();
        let thread = comment_db.save_thread(Thread::new("thread")).await.unwrap();
        let comment = Comment::new(thread.hash.as_str(), "user@mail.com", 17, "content");
        let saved_comment = comment_db.save_comment(comment.clone()).await.unwrap();

        assert_eq!(&saved_comment, &comment);
        assert_eq!(
            comment_db
                .delete_comment(comment.hash.as_str())
                .await
                .unwrap(),
            Some(saved_comment)
        );
        assert_eq!(
            comment_db
                .delete_comment(comment.hash.as_str())
                .await
                .unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_find_all_comments() {
        let mut comment_db = DbWrapper::default();
        let thread = comment_db.save_thread(Thread::new("thread")).await.unwrap();
        let thread_2 = comment_db.save_thread(Thread::new("thread_2")).await.unwrap();
        let _comment = comment_db.save_comment(Comment::new(thread.hash.as_str(), "user@mail.com", 17, "content")).await.unwrap();
        let _comment_2 = comment_db.save_comment(Comment::new(thread.hash.as_str(), "user1@mail.com", 17, "content")).await.unwrap();
        let _comment_3 = comment_db.save_comment(Comment::new(thread_2.hash.as_str(), "user2@mail.com", 17, "content")).await.unwrap();

        assert_eq!(comment_db.find_all_comments(thread.hash.as_str()).await.unwrap().len(), 2);
        assert_eq!(comment_db.find_all_comments(thread_2.hash.as_str()).await.unwrap().len(), 1);
    }
}
