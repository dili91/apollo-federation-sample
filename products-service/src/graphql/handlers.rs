use crate::graphql::{ProductsSchema};
use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub async fn graphql_api(
    schema: web::Data<ProductsSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> std::io::Result<HttpResponse> {
    let api_path = "/graphql/api";
    let source =
        playground_source(GraphQLPlaygroundConfig::new(api_path).subscription_endpoint(api_path));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
