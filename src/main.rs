//mod task1;
mod task2;
mod random_population;
mod selection;
mod knap;
mod crossover;
mod mutation;

fn evaluate_winners(knp: &knap::Knapsack, crossover_rate: f32, mutation_rate: f32, tournament_size: u16, population_size: u16, n: u16){
    let mut population = random_population::init_population(n,population_size);
    let iterations = 10;
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
    println!("evaluation {} \nfinished \n\n", winning_eval);
}

fn main() -> Result<(),()> {
    let out = "out.csv";
    let mutation_rate = 0.01; 
    let tournament_size = 10;
    let population_size = 800;
    let n = 1001;
    let knp = task2::read(out.to_string())?;
    //task1::generate(1001,15000,15000,out.to_string());
    println!("\n"); 
    let mut crossover_rate =  0.01;
    evaluate_winners(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n);
    crossover_rate =  0.02;
    evaluate_winners(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n);
    crossover_rate =  0.03;
    evaluate_winners(&knp,crossover_rate,mutation_rate,tournament_size, population_size, n);

    Ok(())
}
