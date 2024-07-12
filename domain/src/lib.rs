use log::info;

#[derive(Debug)]
pub struct Car {
    pub brand: String,
    pub model: String,
}

pub trait CarRepository {
    fn find_all(&self) -> Vec<Car>;
}

pub struct CarSearchUseCase<'a> {
    pub car_repository: &'a dyn CarRepository,
}

impl CarSearchUseCase<'_> {
    pub fn run(&self) -> Vec<Car> {
        let cars = self.car_repository.find_all();
        info!("searching for all cars, found {:?}", cars);
        return cars;
    }
}
