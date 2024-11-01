// Konfiguracja

use rand_mt::Mt19937GenRand32;
use crate::osobnik::Solution;



pub struct Config {
    pub city_count: i32,
    pub matrix: Vec<Vec<i32>>,
    pub stop_time: u64,
    pub starting_population_size: i32,
    pub mutation_rate: f32,
    pub crossover_factor: f32,
    pub mutation_method: i32,
    pub roulette_ver: i32,
    pub name_of_matrix: String,
    pub seed: u32,
    pub population: Vec<Solution>,
    pub best_solution: Solution,
    pub rng :Mt19937GenRand32,
}
impl Config {
    pub fn new() -> Self {

        //ziarno
        let seed = 12345;

        Config {
            city_count: 0,
            matrix: Vec::new(),
            stop_time: 15,
            starting_population_size: 500,
            mutation_rate: 0.01,
            crossover_factor: 0.8,
            mutation_method: 0,
            roulette_ver: 0,
            name_of_matrix: String::new(),
            seed,
            population: Vec::new(),
            best_solution: Solution::new(Vec::new(),i32::MAX),
            rng: Mt19937GenRand32::new(seed),
        }
    }

    pub fn print_best(&self) {
        println!("Best solution: ");
        println!("{:?}", self.best_solution);
        println!("Solution length: {}", self.best_solution.path_length);
    }
}



