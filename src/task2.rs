use std::error::Error;
use csv::ReaderBuilder;
use crate::knap::{Record,Knapsack,KnapsackBase};

fn insert_base(rec: Record) -> KnapsackBase {
    let base = KnapsackBase {
        max_weight: rec.weight,
        number: rec.price,
        max_size: rec.size
    };
    return base;
}

fn read_file(input_file: String) -> Result<Knapsack, Box<dyn Error>> {
    let mut knp_base =  KnapsackBase {
        max_weight: 0,
        number: 0,
        max_size: 0
    };
    let mut recs = Vec::<Record>::new();
    let mut first_row: bool = true;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(input_file)?;
    for result in rdr.deserialize() {
        let rec: Record = result?;
        if first_row {
            knp_base = insert_base(rec);
            first_row = false;
            continue;
        }
        recs.push(rec);
    }
    let knp = Knapsack {
        base_values: knp_base,
        values: recs 
    };
    Ok(knp)
}

fn new_knapsack() -> Knapsack {
    let base_v = KnapsackBase {
        max_weight: 0,
        number: 0,
        max_size: 0
    };
    return Knapsack {
        base_values: base_v,
        values: Vec::<Record>::new()
    };
}

pub fn read(input_file: String) -> Result<Knapsack, ()>{
    let res = read_file(input_file);
    match res {
        Ok(knp) => {
            println!("Read correctly");
            Ok(knp)
        },
        Err(e) => {
            println!("ERROR");
            Err(())
        },
    }
}
