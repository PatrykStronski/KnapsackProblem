use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Record {
    pub weight: u16,
    pub price: u16,
    pub size: u16
}

pub struct KnapsackBase {
    pub max_weight: u16,
    pub number: u16,
    pub max_size: u16
}

pub struct Knapsack {
    pub base_values: KnapsackBase,
    pub values: Vec<Record>
}


