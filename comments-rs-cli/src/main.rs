use std::sync::Arc;

use comments_rs_core::{traits::{Frontend, UserStore}, Components};
use comments_rs_graphql::GraphQLFrontend;
use futures::join;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

    let mut cmps = Arc::new(Components {
        frontend: Option::None,
        user_store: Option::None
    });

    let frontend = GraphQLFrontend { cmps: cmps.clone() };

    // cmps.frontend = Some(Arc::new(frontend));

    rt.block_on(run(cmps));
}

async fn run(cmps: Arc<Components>) {
    let _res = join!(cmps.frontend.as_ref().unwrap().run());
}
