use comments_rs_graphql_frontend::Query;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use comments_rs_core_frontend::traits::ThreadStore;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn it_works() {
    let query = Query{};

    let thread = query.load("hash");
}
