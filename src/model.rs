use async_graphql::*;

#[derive(Default)]
pub struct QueryRoot;
#[Object]
impl QueryRoot {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn find_users(&self,ctx: &Context<'_>) -> FieldResult<Vec<User>> {
        let rep = &ctx.data_unchecked::<PostgresDB>();
        rep.get_users().await
    }

    async fn find_user(&self,ctx:&Context<'_>,input:FindById) -> FieldResult<User>{
        let rep = &ctx.data_unchecked::<PostgresDB>();
        rep.get_user(input.id).await
    }

    async fn find_likes(&self, ctx:&Context<'_>, input:FindById) -> FieldResult<Vec<Likes>>{
        let rep= &ctx.data_unchecked::<PostgresDB>();
        rep.get_likes(input.id).await
    }

    async fn find_matches(&self, ctx:&Context<'_>)->FieldResult<Vec<Match>>{
        let rep= &ctx.data_unchecked::<PostgresDB>();
        rep.get_matches().await
    }

}
pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>,input: CreateUser) -> FieldResult<User>{
        let rep = &ctx.data_unchecked::<PostgresDB>();
        let new_user = User {
                id: None,
                name: input.name,
            };
        rep.create_user(new_user).await
    }

    async fn like_request(&self, ctx: &Context<'_>, input:CreateLike) -> FieldResult<Likes>{
        let rep = &ctx.data_unchecked::<PostgresDB>();
        rep.like_request(input).await
    }

    async fn accept_like(&self, ctx: &Context<'_>, input:CreateMatch) -> FieldResult<Match>{
        let rep=&ctx.data_unchecked::<PostgresDB>();
        rep.be_match(input.user1_id,input.user2_id).await
    }


}
#[derive(Default)]
#[derive(SimpleObject)]
pub struct MyObject {
    pub a: i32,
    pub b: i32,
    #[graphql(skip)]
    pub c: i32,
}

#[derive(Default)]
#[derive(SimpleObject)]
#[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.
pub struct MyObj {
    pub a: i32,
    pub b: i32,
}

#[ComplexObject]
impl MyObj {
    async fn c(&self) -> i32 {
        self.a + self.b
    }
}

use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres, types::Uuid};

use serde::{Deserialize, Serialize};
pub struct PostgresDB {
    db: Pool<Postgres>,
}
impl PostgresDB {
    pub async fn init() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://doosan:Conestoga1@localhost/test")
            .await;
        let pool = pool.unwrap();
        
        PostgresDB { db: pool }
    }

    //create
    pub async fn create_user(&self, new_user: User) -> Result<User>{
        // let row: (i64,) = sqlx::query_as("SELECT $1")
        // .bind(150_i64)
        // .fetch_one(&pool).await;
        let result = sqlx::query_as::<_,(i32,)>("insert into users (name) values($1) RETURNING id;").bind(new_user.name.clone()).fetch_one(&self.db).await;

        let id = result?;

        Ok(User { id: Some(id.0), name: new_user.name })
    }

    pub async fn like_request(&self, new_request: CreateLike) ->Result<Likes>{
        let result = sqlx::query_as::<_,(i32,)>("insert into likes (sender,reciver) values ($1,$2) returning id;")
        .bind(new_request.sender)
        .bind(new_request.reciver)
        .fetch_one(&self.db).await;

        let id=result?;
        Ok(Likes{
            id:Some(id.0),
            sender:new_request.sender,
            reciver:new_request.reciver
        })
    }
    pub async fn be_match(&self,user1_id:i32,user2_id:i32) -> Result<Match>{
        let result= sqlx::query_as::<_,Match>("insert into matchs (user1_id,user2_id) values ($1,$2) returning id;")
        .bind(user1_id)
        .bind(user2_id)
        .fetch_one(&self.db).await?;
        Ok(result)
    }



    //get
    pub async fn get_users(&self) -> Result<Vec<User>> {
        let result:Vec<User> = sqlx::query_as::<_,User>("select * from users;").fetch_all(&self.db).await?;
        Ok(result)
    }

    pub async fn get_user(&self,id:i32) ->Result<User>{
        let result =  sqlx::query_as::<_,User>("select * from users where id= $1;").bind(id).fetch_one(&self.db).await?;
        Ok(result)
    }

    pub async fn get_likes(&self,id:i32) -> Result<Vec<Likes>>{
        let result = sqlx::query_as::<_,Likes>("select * from likes where sender=$1;").bind(id).fetch_all(&self.db).await?;
        Ok(result)
    }

    pub async fn get_matches(&self)->Result<Vec<Match>>{
        let result = sqlx::query_as::<_,Match>("select * from matchs where;").fetch_all(&self.db).await?;
        Ok(result)
    }



    

}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject,sqlx::FromRow,sqlx::Type)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject,sqlx::FromRow)]
pub struct Likes{
    pub id:Option<i32>,
    pub sender:i32,
    pub reciver:i32
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject,sqlx::FromRow)]
pub struct Match{
    pub id:Option<i32>,
    pub user1_id:i32,
    pub user2_id:i32
}





#[derive(InputObject)]
pub struct CreateUser {
    pub name: String,
}

#[derive(InputObject)]
pub struct CreateMatch{
    pub user1_id:i32,
    pub user2_id:i32
}


#[derive(InputObject)]
pub struct CreateLike{
    pub sender:i32,
    pub reciver:i32
}


#[derive(InputObject)]
pub struct FindById{
    pub id: i32
}