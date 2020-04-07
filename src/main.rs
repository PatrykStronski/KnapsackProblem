mod task1;
mod task2;
mod random_population;
mod selection;
mod knap;
mod crossover;
mod mutation;
mod plot_results;

fn evaluate_winner(knp: &knap::Knapsack, crossover_rate: f32, mutation_rate: f32, tournament_size: u32, population_size: u32, n: u32) -> u32{
    let mut population = random_population::init_population(n,population_size);
    let iterations = 20;
    for _i in 0..iterations {
        population = selection::tournament(population.to_vec(), tournament_size, knp);
        population = crossover::crossover_all(population.to_vec(), crossover_rate);
        mutation::mutate_all(&mut population, mutation_rate);
        if population.len() == 1 || population.len() == 2 {
            break;
        }
    }

    println!("crossover_rate: {}\nmutation_rate {}\npopulation_size {}\ntournament_size{}\n",crossover_rate,mutation_rate,population_size,tournament_size);
    let mut winning_eval = 0;
    for pot_winner in population {
        let value = selection::evaluate(&pot_winner, knp);
        if value > winning_eval {
            winning_eval = value;
        }
    }
    return winning_eval;
}

fn main() -> Result<(),()> {
    let out = "out.csv";
    let n = 1001;
    let knp = task2::read(out.to_string())?;
    let nmb_increments = 10;
    let max_weight = 19000;
    let max_size = 19000;
    task1::generate(1001,max_weight,max_size,out.to_string());

    let mut mutation_rate = 0.01;
    let mut tournament_size = 5;
    let mut population_size = 800;
    let mut crossover_rate: f32 =  0.01;
    let mut increment = 0.0;


    println!("Starting eval for crossover rate\n");

    increment = 0.05;
    let mut caption = format!("crossover_rate {}, mutation_rate {}, tournament {}, popul_size {}","MEASURED",mutation_rate,tournament_size,population_size);
    let mut measured_value = Vec::<f32>::with_capacity(nmb_increments);
    let mut winners = Vec::<u32>::with_capacity(nmb_increments);
    for _i in 0..nmb_increments {
        measured_value.push(crossover_rate);
        winners.push(evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n));
        crossover_rate+=increment;
    }
    for _its in 0..4 {
        crossover_rate =  0.01;
        for i in 0..nmb_increments {
            winners[i]+=evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n);
            crossover_rate+=increment;
        }
    }
    plot_results::plot_results(winners.to_vec(),measured_value.to_vec(),"crossover_rate".to_string(), caption.to_string());

    crossover_rate = 0.25;

    println!("Starting eval for mutation rate\n");
    mutation_rate = 0.001;
    increment = 0.005;
    caption = format!("crossover_rate {}, mutation_rate {}, tournament {}, popul_size {}",crossover_rate,"MEASURED",tournament_size,population_size);
    measured_value = Vec::<f32>::with_capacity(nmb_increments);
    winners = Vec::<u32>::with_capacity(nmb_increments);
    for _i in 0..nmb_increments {
        measured_value.push(mutation_rate);
        winners.push(evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n));
        mutation_rate+=increment;
    }
    for _its in 0..4 {
        mutation_rate = 0.001;
        for i in 0..nmb_increments {
            winners[i]+=evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n);
            mutation_rate+=increment;
        }
    }
    plot_results::plot_results(winners.to_vec(),measured_value.to_vec(),"mutation_rate".to_string(), caption.to_string());

    mutation_rate = 0.02;

    println!("Starting eval for tournament size\n");

    let mut increment_i: u32 = 1;
    tournament_size = 2;
    caption = format!("crossover_rate {}, mutation_rate {}, tournament {}, popul_size {}",crossover_rate,mutation_rate,"MEASURED",population_size);
    measured_value = Vec::<f32>::with_capacity(nmb_increments);
    winners = Vec::<u32>::with_capacity(nmb_increments);
    for _i in 0..nmb_increments {
        measured_value.push(tournament_size as f32);
        winners.push(evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n));
        tournament_size+=increment_i;
    }
    for _its in 0..4 {
        tournament_size = 2;
        for i in 0..nmb_increments {
            winners[i]+=evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n);
            tournament_size+=increment_i;
        }
    }
    plot_results::plot_results(winners.to_vec(),measured_value.to_vec(),"tournament_size".to_string(), caption.to_string());
    tournament_size = 10;

    println!("Starting eval for population size\n");

    increment_i = 100;
    population_size = 100;
    caption = format!("crossover_rate {}, mutation_rate {}, tournament {}, popul_size {}",crossover_rate,mutation_rate,tournament_size,"MEASURED");
    measured_value = Vec::<f32>::with_capacity(nmb_increments);
    winners = Vec::<u32>::with_capacity(nmb_increments);
    for _i in 0..nmb_increments {
        measured_value.push(population_size as f32);
        winners.push(evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n));
        population_size+=increment_i;
    }
    for _its in 0..4 {
        population_size = 100;
        for i in 0..nmb_increments {
            winners[i]+=evaluate_winner(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n);
            population_size+=increment_i;
        }
    }
    plot_results::plot_results(winners.to_vec(),measured_value.to_vec(),"population_size_size".to_string(), caption.to_string());
    population_size = 800;

    Ok(())
}
