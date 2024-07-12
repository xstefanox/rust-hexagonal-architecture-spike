use env_logger::Target;
use LevelFilter::Debug;
use log::LevelFilter;
use Target::Stdout;

use adapter::{CarDealerStub, InMemoryCarRepository};
use domain::CarSearchUseCase;

fn main() {
    env_logger::Builder::from_default_env()
        .target(Stdout)
        .filter_level(Debug)
        .parse_default_env()
        .init();

    let repository = InMemoryCarRepository {};
    let car_dealer = CarDealerStub();
    let search_cars = CarSearchUseCase {
        car_repository: &repository,
        car_dealer: &car_dealer,
    };

    let cars = search_cars.run();

    println!("{:?}", cars);
}
