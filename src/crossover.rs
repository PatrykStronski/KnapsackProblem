use rand::Rng;

fn should_crossover(crossover_rate: f32) -> bool {
    if crossover_rate <= rand::thread_rng().gen_range(0.0, 1.0) {
        return true;
    } else {
        return false;
    }
}

fn choose_exchange_point(max: u32) -> u32 {
    return rand::thread_rng().gen_range(1,max);
}

fn exchange_chromosomes(individual1: Vec<bool>, individual2: Vec<bool>, exchange_point: u32) -> Vec<Vec<bool>> {
    let ind_len = individual1.len() as u32;
    let mut children = Vec::<Vec<bool>>::with_capacity(2);
    let mut child1 = Vec::<bool>::with_capacity(ind_len as usize);
    let mut child2 = Vec::<bool>::with_capacity(ind_len as usize);
    for i in 0..exchange_point {
        child1.push(individual1[i as usize]);
        child2.push(individual2[i as usize]);
    }
    for i in exchange_point..ind_len {
        child1.push(individual2[i as usize]);
        child2.push(individual1[i as usize]);
    }
    children.push(child1);
    children.push(child2);
    return children;
}

fn crossover(individual1: Vec<bool>, individual2: Vec<bool>, crossover_rate: f32) -> Vec<Vec<bool>> {
    if should_crossover(crossover_rate) {
        let exchange_point = choose_exchange_point(individual1.len() as u32);
        return exchange_chromosomes(individual1, individual2, exchange_point);
    }
    let mut children = Vec::<Vec<bool>>::with_capacity(2);
    children.push(individual1);
    children.push(individual2);
    return children;
}

pub fn crossover_all(mut individuals: Vec<Vec<bool>>, crossover_rate: f32) -> Vec<Vec<bool>> {
    let number = individuals.len() as u32;
    rand::thread_rng().shuffle(&mut individuals);
    let mut children = Vec::<Vec<bool>>::with_capacity(individuals.len());
    let mut iter: u32 = 0;
    while iter < number-1 {
        let n = iter as usize;
        let i1 = individuals[n].to_vec();
        let i2 = individuals[n+1].to_vec();
        let crossover_off = crossover(i1, i2, crossover_rate);
        children.push(crossover_off[0].to_vec());
        children.push(crossover_off[1].to_vec());
        iter=iter+2;
    }
    return children;
}
