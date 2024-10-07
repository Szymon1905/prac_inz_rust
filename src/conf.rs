pub struct Config {
    pub city_count: i32,
    pub matrix: Vec<Vec<i32>>,
    pub stop_time: i32,
    pub starting_population_size: i32,
    pub mutation_rate: f32,
    pub crossover_factor: f32,
    pub mutation_method: i32,
    pub roulette_ver: i32,
    pub name_of_matrix: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            city_count: 0,
            matrix: Vec::new(),
            stop_time: 10,
            starting_population_size: 500,
            mutation_rate: 0.01,
            crossover_factor: 0.8,
            mutation_method: 0,
            roulette_ver: 0,
            name_of_matrix: String::new(),
        }
    }
}

