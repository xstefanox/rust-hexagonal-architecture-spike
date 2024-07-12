use log::debug;

use domain::{Car, CarDealer, CarRepository, Purchase, User};

pub struct InMemoryCarRepository();

impl CarRepository for InMemoryCarRepository {
    fn find_all(&self) -> Vec<Car> {
        let result = vec![
            Car {
                brand: "Ferrari".to_string(),
                model: "Portofino".to_string(),
            }
        ];

        debug!("searching for all cars, found {:?}", result);

        result
    }
}

pub struct CarDealerStub();

impl CarDealer for CarDealerStub {
    fn buy(&self, car: &Car, user: &User) -> Purchase {
        debug!("purchasing {:?} for {:?}", car, user);
        return Purchase::Done;
    }
}
