use near_sdk::{log, near};

#[near(contract_state)]
pub struct Contract {
    greetings: near_sdk::store::Vector<String>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            greetings: near_sdk::store::Vector::new(b"g"),
        }
    }
}

#[near]
impl Contract {
    pub fn get_greeting(&self) -> String {
        self.greetings.get(1).unwrap().clone()
    }
}
