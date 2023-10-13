use axum::{routing::get, Router};

mod env;
use env::ENV;

mod api {
    pub mod todo {
        pub mod handlers;
    }
}

use api::todo;

#[derive(Clone, Debug)]
pub struct Context {
    pub foo: String,
}

#[tokio::main]
async fn main() {
    let context = Context {
        foo: "bar".to_owned(),
    };

    let app = Router::new().nest(
        "/api",
        Router::new()
            .nest(
                "/todo",
                Router::new()
                    .route(
                        "/",
                        get(todo::handlers::get_all_todos)
                            .post(todo::handlers::create_todo),
                    )
                    .nest(
                        "/:id",
                        Router::new().route(
                            "/",
                            get(todo::handlers::get_todo)
                                .put(todo::handlers::update_todo)
                                .delete(todo::handlers::delete_todo),
                        ),
                    ),
            )
            .with_state(context),
    );

    let address = format!("{}:{}", ENV.host, ENV.port);

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
