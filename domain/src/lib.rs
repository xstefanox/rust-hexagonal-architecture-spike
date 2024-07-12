use log::{info, warn};

#[derive(Debug, Clone)]
pub struct Car {
    pub brand: String,
    pub model: String,
}

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
}

pub trait CarRepository {
    fn find_all(&self) -> Vec<Car>;
}

pub enum Purchase {
    Done,
    Rejected,
}

pub trait CarDealer {
    fn buy(&self, car: &Car, user: &User) -> Purchase;
}

pub struct CarSearchUseCase<'a> {
    pub car_repository: &'a dyn CarRepository,
    pub car_dealer: &'a dyn CarDealer,
}

impl CarSearchUseCase<'_> {
    pub fn run(&self) -> Vec<Car> {
        let user = User {
            id: 123,
            first_name: "Stefano".to_string(),
            last_name: "Stefano".to_string(),
        };

        let cars = self.car_repository.find_all();

        let result = self.car_dealer.buy(&cars[0], &user);

        match result {
            Purchase::Done => info!("{:?} purchased car {:?}", user, cars[0]),
            Purchase::Rejected => warn!("{:?} could not purchase {:?}", user, cars[0]),
        }

        return cars;
    }
}
