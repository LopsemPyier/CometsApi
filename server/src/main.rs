use std::env;
use dotenv::dotenv;
use async_graphql::{EmptySubscription, Schema};

use graphql::{ContextData, QueryRoot, MutationRoot};
use utils::auth::get_jwt_payload;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use actix_web::{guard, middleware, web, App, HttpRequest, HttpServer, HttpResponse, Responder};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn ping(_req: HttpRequest) -> impl Responder {
    format!(
        "I am healthy: {} v{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    )
}

type SchemaWeb = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn index(schema: web::Data<SchemaWeb>, req: HttpRequest, gql_request: Request) -> Response {
    let context_token = get_jwt_payload(req);
    let mut request = gql_request.into_inner();
    if let Some(token) = context_token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}


#[actix_web::main]
async fn main() -> Result<()> {
	dotenv().ok();
    env_logger::init();


	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host = env::var("HOST").expect("HOST is not set");
    let port = env::var("PORT").expect("PORT is not set");

	let db = db::Database::new(&database_url).await?;

	let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
		.data(ContextData { db })
		.finish();

	println!("Graphiql Playground: http://localhost:8000/graphiql");

	let server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/graphiql").guard(guard::Get()).to(index_playground))
            .route("/ping", web::get().to(ping))
    });

    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}