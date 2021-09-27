mod controllers;
mod libs;

use actix_web::{App, HttpServer};

use controllers::graphql::use_graphql;
use libs::models::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .configure(use_graphql)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
