use rand::Rng;

fn should_crossover(crossover_rate: f32) -> bool {
    if crossover_rate <= rand::thread_rng().gen_range(0.0, 1.0) {
        return true;
    } else {
        return false;
    }
}

fn choose_exchange_point(max: u16) -> u16 {
    return rand::thread_rng().gen_range(1,max);
}

fn exchange_chromosomes(individual1: Vec<bool>, individual2: Vec<bool>, exchange_point: u16) -> Vec<bool> {
    let ind_len = individual1.len() as u16;
    let mut child = Vec::<bool>::with_capacity(ind_len as usize);
    for i in 0..exchange_point {
        child.push(individual1[i as usize]);
    }
    for i in exchange_point..ind_len {
        child.push(individual2[i as usize]);
    }
    return child;
}

fn crossover(individual1: Vec<bool>, individual2: Vec<bool>, crossover_rate: f32) -> Vec<bool> {
    if ! should_crossover(crossover_rate) { 
        return individual1; 
    }
    let exchange_point = choose_exchange_point(individual1.len() as u16);
    return exchange_chromosomes(individual1, individual2, exchange_point);        
}

fn calculate_children_capacity(number: u16) -> u16 {
    let res: f32 = (number as f32) / 2.0;
    if res.fract() != 0.0 {
        return (res + 1.0) as u16;
    }
    return res as u16;
}

pub fn crossover_all(mut individuals: Vec<Vec<bool>>, crossover_rate: f32) -> Vec<Vec<bool>> {
    let number = individuals.len() as u16;
    rand::thread_rng().shuffle(&mut individuals);
    let mut children = Vec::<Vec<bool>>::with_capacity(calculate_children_capacity(number) as usize);
    let mut iter: u16 = 0;
    while iter < number-1 {
        let n = iter as usize;
        let i1 = individuals[n].to_vec();
        let i2 = individuals[n+1].to_vec();
        children.push(crossover(i1, i2, crossover_rate));
        iter=iter+2;
    }
    return children;
}
