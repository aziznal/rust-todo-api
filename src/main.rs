use axum::Router;

mod env;
use env::ENV;

mod api {
    pub mod todo {
        pub mod handlers;
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub foo: String,
}

#[tokio::main]
async fn main() {
    let context = Context {
        foo: "bar".to_owned(),
    };

    let app = Router::new().nest("/api", api::todo::handlers::create_router(&context));

    let address = format!("{}:{}", ENV.host, ENV.port);

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
