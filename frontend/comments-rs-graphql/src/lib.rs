use comments_rs_core_frontend::{
    error::Error,
    structs::Thread,
    traits::{StoreResult, ThreadStore},
};
use graphql_client::{GraphQLQuery, Response};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(val: &str);
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../backend/comments-rs-graphql/schema.graphql",
    query_path = "query/thread_query.graphql"
)]
pub struct Query;

pub struct GraphqlStore {
    base_url: String,
}

impl GraphqlStore {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

impl ThreadStore for GraphqlStore {
    fn load(&self, hash: &str) -> StoreResult<Thread> {
        let hash = hash.to_string();
        let base = self.base_url.clone();

        Box::pin(async move {
            let request_body = Query::build_query(query::Variables { hash: Some(hash) });

            let client = reqwest::Client::new();

            let url = format!("{}/", base);

            log(&format!("Url: {}", &url));

            let mut res = client
                .post(url)
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
                .await
                .unwrap();

            let response_body: Response<Thread> = res.json().await.unwrap();
            Ok(response_body.data)
        })
    }
}
