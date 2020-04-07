use rand::Rng;

fn create_individual(n_items: u32) -> Vec<bool> {
    let mut ind = Vec::with_capacity(n_items as usize);
    for _i in 0..n_items {
        let nmb = rand::thread_rng().gen_range(0,2);
        if nmb == 1 {
            ind.push(true);
        } else {
            ind.push(false);
        }
    }
    return ind;
}

/*
 * n_items - number of items from which the subset tobe packed into the knapsackis selected. numberof items generated in task 1
 * size - population size
 * */
pub fn init_population(n_items: u32, size: u32) -> Vec<Vec<bool>> {
    let mut population = Vec::<Vec<bool>>::with_capacity(size as usize);
    for _i in 0..size {
        population.push(create_individual(n_items));
    }
    return population;
}
