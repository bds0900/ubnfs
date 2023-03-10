use async_graphql::{Schema, EmptyMutation, EmptySubscription, MergedObject};

pub mod model;
pub mod user;
pub mod matched;
pub use model::{QueryRoot, Mutation, MyObj, MyObject,PostgresDB};

#[derive(MergedObject, Default)]
pub struct Query(QueryRoot, MyObject);

pub type UbnfsSchema = Schema<Query, Mutation, EmptySubscription>;
pub type MyObjSchema = Schema<MyObj, EmptyMutation, EmptySubscription>;
pub type MyObjectSchema = Schema<MyObj, EmptyMutation, EmptySubscription>;