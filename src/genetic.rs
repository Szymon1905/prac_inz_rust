use crate::conf::Config;
use crate::osobnik::Solution;

fn print_best(config: Config){
    println!("best solution {:?}", config.best_solution.cities);
}

fn timer(){

}

fn generate_starting_population(){

}

pub(crate) fn calculate_path_length(solution: &mut Solution, config: &mut Config) -> i32 { //todo test this
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