use actix_web::{web, App, HttpServer};
use async_graphql_actix_web::Response;
use server1::schema::{get_schema, Server1Schema};

async fn graphql(
    schema: web::Data<Server1Schema>,
    request: async_graphql_actix_web::Request,
) -> Response {
    schema.execute(request.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(get_schema().clone())
            .route("/graphql", web::post().to(graphql))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
