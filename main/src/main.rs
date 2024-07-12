use env_logger::Target;
use LevelFilter::Info;
use log::LevelFilter;
use Target::Stdout;

use domain;
use domain::CarSearchUseCase;

fn main() {
    env_logger::Builder::from_default_env()
        .target(Stdout)
        .filter_level(Info)
        .parse_default_env()
        .init();

    let repository = adapter::InMemoryCarRepository {};
    let search_cars = CarSearchUseCase {
        car_repository: &repository,
    };

    let cars = search_cars.run();

    println!("{:?}", cars);
}
