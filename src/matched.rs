use async_graphql::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject,sqlx::FromRow)]
pub struct Matched{
    pub id:Option<i32>,
    pub user1_id:i32,
    pub user2_id:i32
}

