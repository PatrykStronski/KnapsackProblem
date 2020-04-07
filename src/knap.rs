use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Record {
    pub weight: u32,
    pub size: u32,
    pub price: u32

}

pub struct KnapsackBase {
    pub max_weight: u32,
    pub number: u32,
    pub max_size: u32
}

pub struct Knapsack {
    pub base_values: KnapsackBase,
    pub values: Vec<Record>
}
