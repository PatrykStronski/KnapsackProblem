mod task1;
mod task2;
mod random_population;
mod selection;
mod knap;
mod crossover;
mod mutation;

fn main() -> Result<(),()> {
    let out = "out.csv";
    let crossover_rate =  0.01;
    let mutation_rate = 0.01; 
    let iterations = 10;
    let population = random_population::init_population(1001,1000);

    //task1::generate(1001,15000,15000,out.to_string());
    let knp = task2::read(out.to_string())?;
    for _i in 0..iterations {
        let mut winners = selection::tournament(population, 10, knp); 
        crossover::crossover_all(&mut winners, crossover_rate);
        mutation::mutate_all(&mut winners, mutation_rate);
    }
    Ok(())
}
