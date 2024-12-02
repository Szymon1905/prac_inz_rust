// Konfiguracja

use rand_mt::Mt19937GenRand32;
use crate::solution::Solution;



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
    pub operation_count:i32,
}
impl Config {
    pub fn new() -> Self {

        //ziarno dla generatora liczb losowych
        let seed = 12345;

        Config {
            city_count: 0, // liczba miast dla danej instacji TSP
            matrix: Vec::new(), //macierz sąsiedztwa
            stop_time: 10, // warunek stopu czasowy
            starting_population_size: 500, // wielkosć popualcji startowej
            mutation_rate: 0.01, //współcznnik mutacji
            crossover_factor: 0.8, //współcznnik krzyżowania
            mutation_method: 0, //metoda mutacji
            roulette_ver: 0, // wersja ruletki, opcjonalnie dodam drugą z c++
            name_of_matrix: String::new(),
            seed, // ziarno
            population: Vec::new(), // vectora na populację
            best_solution: Solution::new(Vec::new(),i32::MAX), // aktualnie najlpesze rozwiązaie, poźniej zwracane
            rng: Mt19937GenRand32::new(seed), // generator liczb losowych
            operation_count: 0, // meirnik liczby operacji do testów wydajności
        }
    }

    // funkcja wypisująca najlepsze rozwiązanie
    pub fn print_best(&self) {
        println!("Best solution: ");
        println!("{:?}", self.best_solution);
        println!("Solution length: {}", self.best_solution.path_length);
    }
}



