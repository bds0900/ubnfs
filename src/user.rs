use async_graphql::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject,sqlx::FromRow,sqlx::Type)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
}