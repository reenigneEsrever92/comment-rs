use std::env;

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, futures_util::TryFutureExt};
use comments_rs_core::{traits::Frontend, error::{CommentError, Error}};

struct RootQuery;

#[Object]
impl RootQuery {
    async fn root(&self) -> &str {
        "test"
    }
}

pub struct GraphQLFrontend;

impl Frontend for GraphQLFrontend {
    fn run(&self) -> Box<dyn std::future::Future<Output = Result<(), Error>>> {
        let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

        let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription).finish();

        println!("Playground: http://{}", listen_addr);

        let mut app = tide::new();

        app.at("/").post(async_graphql_tide::graphql(schema));

        Box::new(app.listen(listen_addr).map_err(|_| Error::NewtorkError))
    }
}

#[cfg(test)]
mod test {
    use comments_rs_core::traits::Frontend;
    use crate::GraphQLFrontend;


    // #[tokio::test]
    // async fn test_find_all_comments() {
    //     let frontend = GraphQLFrontend {};
    //     frontend.run().await.unwrap();
    // }

}