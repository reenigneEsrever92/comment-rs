use comments_rs_core_frontend::traits::ThreadStore;
use comments_rs_graphql_frontend::GraphqlStore;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

#[wasm_bindgen_test]
async fn it_works() {
    let graphql_store = GraphqlStore::new("http://localhost:8080");

    let result = graphql_store.load("hash").await;

    assert!(result.is_ok());
}
