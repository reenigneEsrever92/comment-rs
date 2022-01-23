use std::sync::Arc;

use comments_rs_core_backend::traits::Frontend;
use comments_rs_graphql_backend::GraphQLFrontend;
use comments_rs_memdb_backend::MemDB;
use futures::join;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

    let memdb = Arc::new(MemDB::default());
    let frontend = Box::new(GraphQLFrontend { user_store: memdb });

    rt.block_on(run(frontend));
}

async fn run(frontend: Box<dyn Frontend>) {
    let _res = join!(frontend.run());
}
