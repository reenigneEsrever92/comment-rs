use std::{cell::RefCell, env, pin::Pin, sync::Arc};

use async_graphql::{
    futures_util::TryFutureExt, EmptyMutation, EmptySubscription, Object, OutputType, Schema,
    SimpleObject,
};
use comments_rs_core::{
    data::User,
    error::{CommentError, Error},
    traits::{Frontend, UserStore},
    Components,
};

pub struct RootQuery {
    user_store: Arc<dyn UserStore + Sync + Send>,
}

#[derive(SimpleObject)]
pub struct GraphQLUser {
    email: String,
}

#[Object]
impl RootQuery {
    async fn root(&self) -> &str {
        "test"
    }

    async fn users(&self) -> Vec<GraphQLUser> {
        self.user_store
            .find_all_users()
            .await
            .unwrap()
            .into_iter()
            .map(|user| Into::<GraphQLUser>::into(user))
            .collect()
    }
}

pub struct GraphQLFrontend {
    pub cmps: Arc<Components>,
}

impl From<User> for GraphQLUser {
    fn from(u: User) -> Self {
        GraphQLUser { email: u.email }
    }
}

impl Frontend for GraphQLFrontend {
    fn run(&self) -> Pin<Box<dyn std::future::Future<Output = Result<(), Error>>>> {
        let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

        let schema = Schema::build(
            RootQuery {
                user_store: self.cmps.user_store.as_ref().unwrap().clone(),
            },
            EmptyMutation,
            EmptySubscription,
        )
        .finish();

        println!("Hostet at: http://{}", listen_addr);

        let mut app = tide::new();

        app.at("/").post(async_graphql_tide::graphql(schema));

        Box::pin(app.listen(listen_addr).map_err(|_| Error::NewtorkError))
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use crate::GraphQLFrontend;
    use comments_rs_core::{traits::Frontend, Components};
    use comments_rs_memdb::MemDB;
    use graphql_client::GraphQLQuery;
    use reqwest::Response;
    use serde_json::Value;
    use tokio::select;

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "schema.graphql",
        query_path = "test/root-query.graphql",
        response_derives = "Debug"
    )]
    pub struct RootQuery;

    #[tokio::test]
    async fn test_find_all_comments() {
        let user_store = MemDB::default();

        let cmps = Components {
            frontend: None,
            user_store: Some(Arc::new(user_store)),
        };

        let frontend = GraphQLFrontend { cmps: Arc::new(cmps) };

        let request_body = RootQuery::build_query(root_query::Variables {});

        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:8000/")
            .json(&request_body)
            .send();

        select! {
            res = response => {
                let resp: Response = res.unwrap();
                let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str()).unwrap();

                assert_eq!(&json["data"]["root"], "test");
            },
            _server = frontend.run() => panic!("Server stopped before request returned")
        };
    }
}
