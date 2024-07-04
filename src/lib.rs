use near_sdk::near;

#[near(contract_state)]
pub struct Contract {
    greetings: near_sdk::store::UnorderedSet<String>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            greetings: near_sdk::store::UnorderedSet::new(b"g"),
        }
    }
}

#[near]
impl Contract {
    pub fn get_greeting(&self) -> bool {
        self.greetings.contains("host")
    }
}
