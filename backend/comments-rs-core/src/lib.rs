use std::sync::Arc;

use traits::{Frontend, UserStore};

pub mod data;
pub mod error;
pub mod traits;

pub struct Components {
    pub frontend: Option<Arc<dyn Frontend + Send + Sync>>,
    pub user_store: Option<Arc<dyn UserStore + Send + Sync>>
}