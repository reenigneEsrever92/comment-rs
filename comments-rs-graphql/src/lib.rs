use std::{env, pin::Pin, sync::Arc};

use async_graphql::{
    futures_util::TryFutureExt, Context, EmptyMutation, EmptySubscription, Object,
    Schema, SimpleObject,
};
use comments_rs_core::{
    data::User,
    error::Error,
    traits::{Frontend, UserStore},
};

pub struct Query;

#[derive(SimpleObject)]
pub struct GraphQLUser {
    name: String,
    email: String,
}

#[Object]
impl Query {
    async fn users(&self, ctx: &Context<'_>) -> Vec<GraphQLUser> {
        ctx.data::<Arc<dyn UserStore>>()
            .unwrap()
            .find_all_users()
            .await
            .unwrap()
            .into_iter()
            .map(|user| Into::<GraphQLUser>::into(user))
            .collect()
    }
}

pub struct GraphQLFrontend {
    pub user_store: Arc<dyn UserStore>,
}

impl From<User> for GraphQLUser {
    fn from(u: User) -> Self {
        GraphQLUser { name: u.name, email: u.email }
    }
}

impl Frontend for GraphQLFrontend {
    fn run(&self) -> Pin<Box<dyn std::future::Future<Output = Result<(), Error>>>> {
        let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(self.user_store.clone())    
            .finish();

        println!("Hostet at: http://{}", listen_addr);

        let mut app = tide::new();

        app.at("/").post(async_graphql_tide::graphql(schema));

        Box::pin(app.listen(listen_addr).map_err(|_| Error::NewtorkError))
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::GraphQLFrontend;
    use comments_rs_core::{traits::{Frontend, UserStore}, data::User};
    use comments_rs_memdb::MemDB;
    use graphql_client::GraphQLQuery;
    use reqwest::Response;
    use serde_json::Value;
    use tokio::select;

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "schema.graphql",
        query_path = "test/users-query.graphql",
        response_derives = "Debug"
    )]
    pub struct Query;

    #[tokio::test]
    async fn test_find_all_comments() {
        let mut memdb = MemDB::default();

        let _user = memdb.save_user(User::new("test@mail.com", "test")).await.expect("Could not save user!");
        let _user1 = memdb.save_user(User::new("test2@mail.com", "test2")).await.expect("Could not save user!");

        let frontend = GraphQLFrontend {
            user_store: Arc::new(memdb),
        };

        let request_body = Query::build_query(query::Variables {});

        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:8000/")
            .json(&request_body)
            .send();

        select! {
            res = response => {
                let resp: Response = res.unwrap();
                let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str()).unwrap();

                assert_eq!(&json["data"]["users"][0]["name"], "test");
                assert_eq!(&json["data"]["users"][0]["email"], "test@mail.com");
                assert_eq!(&json["data"]["users"][1]["name"], "test2");
                assert_eq!(&json["data"]["users"][1]["email"], "test2@mail.com");
            },
            _server = frontend.run() => panic!("Server stopped before request returned")
        };
    }
}
