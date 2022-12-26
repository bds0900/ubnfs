use async_graphql::{Schema, EmptyMutation, EmptySubscription};

mod model;
pub use model::{QueryRoot, Mutation, MyObj, MyObject,PostgresDB};

pub type UbnfsSchema = Schema<QueryRoot, Mutation, EmptySubscription>;
pub type MyObjSchema = Schema<MyObj, EmptyMutation, EmptySubscription>;
pub type MyObjectSchema = Schema<MyObj, EmptyMutation, EmptySubscription>;