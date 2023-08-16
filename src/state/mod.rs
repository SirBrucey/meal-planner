mod ingredients;

use ingredients::Ingredients;

pub struct State {
    ingredients: Ingredients,
}

impl State {
    pub fn new() -> Self {
        Self {
            ingredients: Ingredients::new(),
        }
    }
}
