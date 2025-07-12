use axum::{routing::get, Json, Router};
use diesel::prelude::*;
use diesel_async::{RunQueryDsl};

use api::config::database::establish_connection;
use api::models::tag::Tag;
use api::schema::tag::dsl::*;

#[derive(serde::Serialize)]
pub struct ApiResponse {
    pub data: Vec<Tag>
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<ApiResponse> {
    let connection = &mut establish_connection().await;
    let results = tag
        .select(Tag::as_select())
        .load(connection)
        .await
        .unwrap();

    let response = ApiResponse {
        data: results
    };

    return Json(response)
}
