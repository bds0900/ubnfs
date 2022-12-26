use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result, HttpRequest};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use Ubnfs::{QueryRoot, Mutation, MyObj, MyObject, UbnfsSchema, MyObjectSchema, MyObjSchema,PostgresDB,Query};


async fn index(schema: web::Data<UbnfsSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000")
                .finish(),
        ))
}

async fn index_ws(
    schema: web::Data<UbnfsSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // console app
    let schema = Schema::build(Query::default(), Mutation, EmptySubscription)
        .data(PostgresDB::init().await)
        .finish();
        
    let res = schema.execute("{ add(a: 10, b: 20) }").await;
    let json = serde_json::to_string(&res);
    println!("{:?}", json);

    // console app
    let schema1 = Schema::new(MyObject{ a: 10, b:20,c:20 }, EmptyMutation, EmptySubscription);
    let res = schema1.execute("{ a,b}").await.into_result().unwrap().data;
    println!("{:?}",res);

    // console app
    let schema2 = Schema::new(MyObj{a:10,b:20}, EmptyMutation, EmptySubscription);
    let res = schema2.execute("{ a,b,c}").await.into_result().unwrap().data;
    println!("{:?}",res);


    // web app
    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
            .service(web::resource("/").guard(guard::Get()).to(index_ws))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
