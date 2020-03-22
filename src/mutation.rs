use rand::Rng;

fn choose_random_gene(max: usize, taken: Vec<usize>) -> usize {
    let mut index = rand::thread_rng().gen_range(0,max) as usize;
    let mut safety_retries = 10;
    while taken.contains(&index) && safety_retries > 0 {
        index = rand::thread_rng().gen_range(1,max) as usize;
        if safety_retries == 0 {
            break;
        }
        safety_retries = safety_retries - 1;
    }
    return index;
}

pub fn mutate(individual1: &mut Vec<bool>, mutation_rate: f32) {
    let nmb_genes_to_mutate = ((mutation_rate * 1000.0) - (mutation_rate * 1000.0).fract()) as u16;
    let max_range = individual1.len();
    let mut gene_indexes = Vec::<usize>::with_capacity(nmb_genes_to_mutate as usize);
    for _i in 0..nmb_genes_to_mutate {
        let gene_ind = choose_random_gene(max_range, gene_indexes.to_vec());
        gene_indexes.push(gene_ind as usize);
        individual1[gene_ind] = ! individual1[gene_ind];
    }
}

pub fn mutate_all(individuals: &mut Vec<Vec<bool>>, mutation_rate: f32) {
    for person in individuals {
        mutate(person,mutation_rate);
    }
}
