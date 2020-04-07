use crate::selection::{tournament,evaluate};
use crate::crossover::{crossover_all};
use crate::knap::{Knapsack};

struct GraphNode {
        children: Vec<GraphNode>,
        value: u32
}

const TOURNAMENTS: [u32;10] = [3,4,5,6,7,8,9,10,11,12];
const CROSSOVER_RT: f32 = 0.05;

fn create_graph(path_size: usize, population: Vec<Vec<bool>>, task_set: &Knapsack, curr_tournament: u32, stop:u16) -> GraphNode {
    let mut paths = Vec::<GraphNode>::with_capacity(path_size);
    let mut children = tournament(population.to_vec(), curr_tournament, &task_set);
    children = crossover_all(children.to_vec(),CROSSOVER_RT);
    if stop > 0 {
        for i in 0..path_size {
            paths.push(create_graph(path_size,children.to_vec(),task_set, TOURNAMENTS[i],stop-1));
        }
    } else {
        paths = Vec::<GraphNode>::with_capacity(0);
    }
    let mut winning_eval = 0;
    for pot_winner in children {
        let value = evaluate(&pot_winner, task_set);
        if value > winning_eval {
            winning_eval = value;
        }
    }
    return GraphNode {
        children: paths,
        value:  winning_eval
    };
}

fn find_best_value(gn: &GraphNode) -> u32{
    let best_list = find_best_from_list(&gn.children);
    if best_list > gn.value {
        return best_list;
    } else {
        return gn.value;
    }
}

fn find_best_from_list(gns: &Vec<GraphNode>) -> u32 {
    let mut best: u32 = 0;
    for g in gns {
        let tmp = find_best_value(&g);
        if tmp > best {
            best = tmp;
        }
    }
    return best;
}

pub fn ant_evaluate(mut path_size: usize, population: Vec<Vec<bool>>, task_set: &Knapsack, stop: u16) -> u32 {
    if path_size > TOURNAMENTS.len() {
        path_size = TOURNAMENTS.len();
    }
    let mut paths = Vec::<GraphNode>::with_capacity(path_size);
    for i in 0..path_size {
        paths.push(create_graph(path_size,population.to_vec(), task_set, TOURNAMENTS[i],stop-1));
    }
    return find_best_from_list(&paths);
}
