use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

use serde::{Deserialize, Serialize};

mod env;
use env::ENV;

mod helpers;
use helpers::create_route::create_route;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(create_root_router());

    let address = format!("{}:{}", ENV.host, ENV.port);

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Debug)]
struct RootResponse {
    message: String,
}

#[derive(Deserialize, Debug)]
struct NewItem {
    name: String,
}

fn create_root_router() -> Router {
    async fn root_get(State(state): State<&str>, Path(user_id): Path<String>) -> impl IntoResponse {
        let response = RootResponse {
            message: "Yo".to_owned(),
        };

        println!("Got the following state: {}", state);
        println!("Got the following user id: {}", user_id);

        (StatusCode::OK, Json(response))
    }

    async fn root_post(Json(payload): Json<NewItem>) -> impl IntoResponse {
        println!("received payload {:?}", payload);

        (StatusCode::CREATED, Json("Done!"))
    }

    async fn root_put() -> &'static str {
        "(PUT) Up and axum!"
    }

    async fn root_delete() -> &'static str {
        "(DELETE) Up and axum!"
    }

    create_route(
        "/api/:id",
        get(root_get)
            .with_state("here's your state")
            .post(root_post)
            .put(root_put)
            .delete(root_delete),
    )
}
