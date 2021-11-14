use actix_web::{web, App, HttpServer};
use async_graphql_actix_web::Response;
use server2::schema::{get_schema, Server2Schema};

async fn graphql(
    schema: web::Data<Server2Schema>,
    request: async_graphql_actix_web::Request,
) -> Response {
    schema.execute(request.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(get_schema())
            .route("/graphql", web::post().to(graphql))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
