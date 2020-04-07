use crate::knap::{Knapsack};

pub fn evaluate(individual: &Vec<bool>, task_set: &Knapsack) -> u32 {
    let mut sum_price: u32 = 0;
    let mut sum_weight: u32 = 0;
    let mut sum_size: u32 = 0;

    for x in 0..individual.len() {
        if individual[x] == true {
            sum_price += task_set.values[x].price;
            sum_weight += task_set.values[x].weight;
            sum_size += task_set.values[x].size;

            if sum_weight > task_set.base_values.max_weight || sum_size > task_set.base_values.max_size {
                return 1;
            }
        }
    }
    return sum_price;
}

fn calculate_tournament_finish(position: u32, max: u32, tour_size: u32) -> u32 {
    let finish = position + tour_size;
    if finish <= max {
        return tour_size;
    }
    return tour_size - (finish - max);
}

fn copy_individual(ind: &Vec<bool>) -> Vec<bool> {
    let mut new_v = Vec::<bool>::with_capacity(ind.len());
    for bit in ind {
        new_v.push(*bit);
    }
    return new_v;
}

pub fn tournament(population: Vec<Vec<bool>>, tournament_size: u32, task_set: &Knapsack) -> Vec<Vec<bool>> {
    let things_nmb = task_set.values.len();
    let mut position: u32 = 0;
    let max = population.len() as u32;
    //let number_of_winners = get_number_of_winners(max as f32, tournament_size as f32);
    let mut winners = Vec::<Vec<bool>>::with_capacity(population.len());
    while position < max {
        let mut best_ind = Vec::<bool>::with_capacity(things_nmb as usize);
        let mut best_money: u32 = 0;
        let finish_tournament = calculate_tournament_finish(position, max, tournament_size);
        for i in 0..finish_tournament {
            let index = (position + i) as usize;
            let current_money: u32 = evaluate(&population[index], task_set);
            if current_money > best_money {
                best_money = current_money;
                best_ind = copy_individual(&population[index]);
            }
        }
        winners.push(best_ind);
        position+=1;
    }
    return winners;
}
