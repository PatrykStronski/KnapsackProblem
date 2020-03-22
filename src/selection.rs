use crate::knap;

fn evaluate(individual: Vec<bool>, task_set: knap::Knapsack) -> u16 {
    let mut sum: u16 = 0;
    for x in 0..individual.len() {
        if individual[x] == true {
            sum += task_set.values[x].price;
        }
    }
    return sum;
}

fn get_number_of_winners(max: f32, tournament_size: f32) -> usize {
    let winners = max / tournament_size;
    let mut number_of_winners: u16 = 0;
    if winners.fract() == 0.0 {
        return winners as usize;
    }
    return (winners - winners.fract() + 1.0) as usize;
}

fn calculate_tournament_finish(position: u16, max: u16, tour_size: u16) -> u16 {
    let finish = position + tour_size;
    if finish <= max {
        return tour_size;
    }
    return tour_size - (finish - max);
}

pub fn tournament(population: Vec<Vec<bool>>, tournament_size: u16, task_set: knap::Knapsack) -> Vec<Vec<bool>> {
    let things_nmb = task_set.values.len();
    let mut position: u16 = 0;
    let max = population.len() as u16;
    let number_of_winners = get_number_of_winners(max as f32, tournament_size as f32);
    let winners = Vec::<Vec<bool>>::with_capacity(number_of_winners);
    while position < max {
        let ind_set = Vec::<Vec<bool>>::with_capacity(tournament_size as usize);
        let mut best_ind = Vec::<bool>::with_capacity(things_nmb as usize);
        let mut best_money: u16 = 0;
        let finish_tournament = calculate_tournament_finish(position, max, tournament_size);
        for i in 0..finish_tournament {
            let index = (position + i) as usize;
            let current_money: u16 = evaluate(population[index],task_set);
            if current_money > best_money {
                best_money = current_money;
                best_ind = population[index];
            }
        }
        winners.push(best_ind);
        position+=tournament_size;
    }
    return winners;
}
