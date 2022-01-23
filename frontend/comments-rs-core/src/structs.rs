use yew::Properties;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    email: String,
    name: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    pub hash: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub user_name: String,
    pub content: String,
}