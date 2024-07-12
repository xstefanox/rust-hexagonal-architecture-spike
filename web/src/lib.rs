use axum::{
    Router
    , routing::post,
};
use axum::extract::State;
use rocket::tokio;

use domain::CarPurchaseUseCase;

pub struct Server<'a> {
    pub purchase_car: CarPurchaseUseCase<'a>,
}

impl Server<'_> {
    pub async fn run(&self) {
        let app = Router::new()
            .route("/buy", post(car_purchase_handler)).with_state(&self.purchase_car);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap();

        self.purchase_car.run();
    }
}

async fn car_purchase_handler(
    State(purchase_car): State<CarPurchaseUseCase<'_>>,
) {
    purchase_car.run();
}
