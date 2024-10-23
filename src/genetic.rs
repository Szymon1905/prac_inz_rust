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

    for mut solution in config.population{
        solution.path_length = calculate_path_length(&mut solution, config);  //todo test this in memory
    }
}

pub(crate) fn calculate_path_length(solution: &mut Solution, config: &mut Config) -> i32 {
    let mut sum_path: i32 = 0;
    let cities_count = solution.cities.len() - 1;
    for i in (0..cities_count){
        sum_path = sum_path + config.matrix[solution.cities[i] as usize][solution.cities[i + 1] as usize];
    }
    // go back to first city
    sum_path += config.matrix[solution.cities[cities_count] as usize][solution.cities[0] as usize];
    return sum_path;
}

fn evaluate_population(){

}

fn custom_parent_choosing_method(){

}

fn choosing_parent_book_method(){

}

fn check_if_contains(){

}

fn ox_crossover(){

}

fn crossover(){

}

fn genetic(){

}