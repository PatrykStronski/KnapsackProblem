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

fn exchange_chromosomes(individual1: &mut Vec<bool>, individual2: &mut Vec<bool>, exchange_point: u16) {
    let ind_len = individual1.len() as u16;
    for i in exchange_point..ind_len {
        let n = i as usize; 
        let ind1 = individual1[n];
        let ind2 = individual2[n];
        individual1[n] = ind2;
        individual2[n] = ind1;
    }
}

fn crossover(individual1: &mut Vec<bool>, individual2: &mut Vec<bool>, crossover_rate: f32) {
    if ! should_crossover(crossover_rate) { 
        return (); 
    }
    let exchange_point = choose_exchange_point(individual1.len() as u16);
    exchange_chromosomes(individual1, individual2, exchange_point);        
}

pub fn crossover_all(individuals: &mut Vec<Vec<bool>>, crossover_rate: f32) {
    let number = individuals.len() as u16;
    rand::thread_rng().shuffle(individuals);
    let mut iter: u16 = 0;
    while iter < number-1 {
        let n = iter as usize;
        let mut i1 = individuals[n].to_vec();
        let mut i2 = individuals[n+1].to_vec();
        crossover(&mut i1, &mut i2, crossover_rate);
        individuals[n] = i1;
        individuals[n+1] = i2;
        iter=iter+2;
    }
}
