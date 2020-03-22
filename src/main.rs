mod task1;
mod task2;
mod random_population;
mod selection;
mod knap;

fn main() -> Result<(),()> {
    let out = "out.csv";
    task1::generate(1001,15000,15000,out.to_string());
    let knp = task2::read(out.to_string())?;
    let population = random_population::init_population(1001,100);
    let winners = selection::tournament(population, 10, knp); 
    
    Ok(())
}
