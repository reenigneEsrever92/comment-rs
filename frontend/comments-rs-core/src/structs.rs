use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    email: String,
    name: String
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Thread {
    pub hash: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub user_name: String,
    pub content: String,
}
