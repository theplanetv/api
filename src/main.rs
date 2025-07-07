use axum::{routing::get, Json, Router};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use uuid::{Uuid};

use api::schema::tag::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = api::schema::tag)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

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
    let connection = &mut establish_connection();
    let results = tag
        .select(Tag::as_select())
        .load(connection)
        .expect("Error loading tags");

    let response = ApiResponse {
        data: results
    };

    return Json(response)
}
