use env_logger::Target;
use LevelFilter::Debug;
use log::LevelFilter;
use Target::Stdout;

use adapter::{CarDealerStub, InMemoryCarRepository};
use domain::CarPurchaseUseCase;
use web::Server;

fn main() {
    env_logger::Builder::from_default_env()
        .target(Stdout)
        .filter_level(Debug)
        .parse_default_env()
        .init();

    Server {
        purchase_car: CarPurchaseUseCase {
            car_repository: &InMemoryCarRepository {},
            car_dealer: &CarDealerStub(),
        }
    }.run();
}
