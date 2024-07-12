use domain::{Car, CarRepository};

pub struct InMemoryCarRepository();

impl CarRepository for InMemoryCarRepository {
    fn find_all(&self) -> Vec<Car> {
        vec![
            Car {
                brand: "Ferrari".to_string(),
                model: "Portofino".to_string(),
            }
        ]
    }
}
