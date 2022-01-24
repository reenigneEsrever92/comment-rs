use comments_rs_core_frontend::{
    error::Error,
    structs::Thread,
    traits::{StoreResult, ThreadStore},
};
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../backend/comments-rs-graphql/schema.graphql",
    query_path = "query/thread_query.graphql"
)]
pub struct Query;

impl ThreadStore for Query {
    fn load(&self, hash: &str) -> StoreResult<Thread> {
        let hash = hash.to_string();

        Box::pin(async {
            let request_body = Query::build_query(query::Variables { hash: Some(hash) }); // TODO

            let client = reqwest::Client::new();
            let mut res = client.post("/").json(&request_body).send().await.unwrap();
            let response_body: Response<Thread> = res.json().await.unwrap();
            Ok(response_body.data)
        })
    }
}
