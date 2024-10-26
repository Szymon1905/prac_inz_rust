use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use rand::seq::SliceRandom;
use crate::conf::Config;
use crate::osobnik::Solution;

fn print_best(config: Config){
    println!("best solution {:?}", config.best_solution.cities);
}

fn timer(){

}

fn generate_starting_population(config: &mut Config){
    // generate cities / cities
    let mut cities: Vec<i32> = Vec::new();
    let city_count = config.city_count;
    println!("{}", city_count);
    for i in (0..city_count) {
        cities.push(i);
    }

    // clear population on restart
    config.population.clear();

    let rng = &mut config.rng;
    let starting_population_size = config.starting_population_size;
    // generating random permutations of cities by the shuffle method and inserting them into the starting population
    for i in (0..starting_population_size) {
        cities.shuffle(rng); // todo check if same as in c++

        let solution = Solution::new(cities.clone(), i32::MAX);
        config.population.push(solution);
    }

    for mut solution in config.population.iter_mut(){
        let sol = solution;
        println!("{:?}", sol.cities);
        let mat = config.matrix.clone();
        sol.path_length = calculate_path_length(sol, mat);  //todo test this in memory
    }
}

pub(crate) fn calculate_path_length(solution: &mut Solution, matrix: Vec<Vec<i32>>) -> i32 {
    let mut sum_path: i32 = 0;
    let cities_count = solution.cities.len() - 1;
    for i in (0..cities_count){
        sum_path = sum_path + matrix[solution.cities[i] as usize][solution.cities[i + 1] as usize];
    }
    // go back to first city
    sum_path += matrix[solution.cities[cities_count] as usize][solution.cities[0] as usize];
    return sum_path;
}

fn evaluate_population(population: &mut Vec<Solution>, global_matrix: Vec<Vec<i32>>, best_solution: &mut Solution) {
    for solution in population.iter_mut() {
        // Calculation of path length
        solution.path_length = calculate_path_length(solution, global_matrix.clone());

        // Compare with the current best solution
        if solution.path_length < best_solution.path_length {
            // Update the best solution for survivability of the best
            *best_solution = solution.clone(); // Assuming Solution implements Clone

            // Print the new best solution's path length
            println!("New best: {}", solution.path_length);
        }
    }
}

fn custom_parent_choosing_method(){

}

fn choosing_parent_book_method(population: &[Solution], mut config: Config) -> Vec<Solution> {
    // Calculate the sum of the inverse path lengths
    let path_sum: f64 = population
        .iter()
        .map(|solution| 1.0 / solution.path_length as f64)
        .sum();

    // Initialize an empty list for selected parents
    let mut chosen_ones: Vec<Solution> = Vec::new();

    // Define a uniform distribution over the range [0, path_sum]
    let distribution = Uniform::new(0.0, path_sum);

    let rng = &mut config.rng;
    // Choosing parents
    for _ in 0..(population.len() / 2) {
        let mut sum = 0.0;
        let los = distribution.sample(rng);

        for solution in population.iter() {
            sum += 1.0 / solution.path_length as f64;
            if sum >= los {
                chosen_ones.push(solution.clone()); // Assuming Solution implements Clone
                break;
            }
        }
    }

    chosen_ones
}

fn check_if_contains(vector: &[i32], liczba: i32) -> bool {
    for &pole in vector {
        if pole == liczba {
            return true;
        }
    }
    return false;
}

fn ox_crossover(parent1: &Solution, parent2 : &Solution ) -> Solution {

    return Solution::new(vec![], 0); // todo palceholder
}

fn crossover(mut config: Config){
    let mut succesor1 : Solution;
    let mut succesor2 : Solution;
    let upper_range = config.population.len() - 1;
    let mut range = Uniform::from(0..=upper_range);
    let crossover_chance = Uniform::from(0.0..=1.0);
    let rng = &mut config.rng;


    let mut new_ones: Vec<Solution> = Vec::new();
    let mut population_temp = config.population;


    for solution in &population_temp {
        let chance: f32 = rng.sample(crossover_chance);

        if chance < config.crossover_factor {
            let rodzic1 = range.sample(rng);
            let rodzic2 = range.sample(rng);

            let parent1 = &population_temp[rodzic1];
            let parent2 = &population_temp[rodzic2];

            succesor1 = ox_crossover(parent1, parent2);
            new_ones.push(succesor1);
        } else {
            let sol = solution.clone();
            new_ones.push(sol);  // Assuming Solution implements Clone
        }
    }

    for solution in &population_temp {
        let szansa = crossover_chance.sample(rng);

        if (szansa < config.crossover_factor) {
            let rodzic1 = range.sample(rng);
            let rodzic2 = range.sample(rng);

            let parent1 = &population_temp[rodzic2];
            let parent2 = &population_temp[rodzic1];

            succesor2 = ox_crossover(parent1, parent2);
            new_ones.push(succesor2);
        } else{
            let sol = solution.clone();
            new_ones.push(sol);
        }

    }
}

pub(crate) fn genetic(config: &mut Config){

    generate_starting_population(config);

    config.best_solution.reset();

    //start = chrono::high_resolution_clock::now();
    //auto stop = start + chrono::seconds(duration);
    // start wątku odliczajacego duration do zatrzymania alogrytmu ( warunek stopu )
    //thread thread_timer(timer, duration); //todo placeholder na timer
    let population = &mut config.population;
    let matrix = config.matrix.clone();
    let best_solution = &mut config.best_solution;
    evaluate_population(population, matrix, best_solution);

    config.print_best();
}