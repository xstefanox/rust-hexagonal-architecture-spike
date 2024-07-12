use axum::{
    Router
    , routing::get,
};
use rocket::tokio;
use domain::CarPurchaseUseCase;

#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub struct Server<'a> {
    pub purchase_car: CarPurchaseUseCase<'a>,
}

impl Server<'_> {
    pub fn run(&self) {
        self.purchase_car.run();
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}