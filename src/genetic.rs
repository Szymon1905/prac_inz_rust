use std::mem;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::time::{Instant, Duration};
use rand::seq::SliceRandom;
use crate::conf::Config;
use crate::osobnik::Solution;
use crate::mutations::mutation;
use rand_mt::Mt19937GenRand32;
fn print_best(config: Config){
    println!("best solution {:?}", config.best_solution.cities);
}

fn timer(){
// todo add timer
}

pub fn shuffle(vec: &mut Vec<i32>, rng: &mut Mt19937GenRand32) {
    let len = vec.len();
    for i in (1..len).rev() {
        let range = Uniform::from(0..=i as i32);  // Ensure `i` is casted correctly to `i32`
        let j = range.sample(rng);
        vec.swap(i, j as usize);  // `j` is cast back to `usize` to match the index type
    }
}

fn generate_starting_population(config: &mut Config){
    // generate cities / cities
    let mut cities: Vec<i32> = Vec::new();
    let city_count = config.city_count;
    println!("city count: {}", city_count);
    for i in (0..city_count) {
        cities.push(i);
    }

    // clear population on restart
    config.population.clear();

    let rng = &mut config.rng;
    let starting_population_size = config.starting_population_size;
    // generating random permutations of cities by the shuffle method and inserting them into the starting population
    for i in (0..starting_population_size) {
        //cities.shuffle(rng); // todo check if same as in c++
        shuffle(&mut cities, rng);
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

    if cities_count > 10 {
        println!("cities count is {}", cities_count);
        println!("{:?}", solution);
    }

    for i in (0..cities_count){
        sum_path = sum_path + matrix[solution.cities[i] as usize][solution.cities[i + 1] as usize];
    }
    // go back to first city
    sum_path += matrix[solution.cities[cities_count] as usize][solution.cities[0] as usize];
    return sum_path;
}

fn evaluate_population(population: &mut Vec<Solution>, global_matrix: &Vec<Vec<i32>>, best_solution: &mut Solution) {
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

fn choosing_parent_book_method(population: &[Solution], rng : &mut Mt19937GenRand32) -> Vec<Solution> {
    // Calculate the sum of the inverse path lengths
    let path_sum: f32 = population
        .iter()
        .map(|solution| 1.0f32 / solution.path_length as f32)
        .sum();

    // Initialize an empty list for selected parents
    let mut chosen_ones: Vec<Solution> = Vec::new();

    // Define a uniform distribution over the range [0, path_sum]
    let distribution = Uniform::new(0.0, path_sum);

    //let rng = &mut config.rng;
    // Choosing parents
    for _ in 0..(population.len() / 2) {
        let mut sum = 0.0f32;
        let los = distribution.sample(rng);

        for solution in population.iter() {
            sum += 1.0f32 / solution.path_length as f32;
            if sum >= los {
                chosen_ones.push(solution.clone());
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

fn ox_crossover(parent1: &Solution, parent2 : &Solution, rng: &mut Mt19937GenRand32) -> Solution {
    let city_count = parent1.cities.len();
    let mut succesor = Solution::new(vec![-1; city_count], i32::MAX);

    let mut random_point = Uniform::from(0 as i32 ..=(city_count - 2) as i32);
    let mut point1 = rng.sample(random_point);
    let mut point2 = rng.sample(random_point);

    if (point1 > point2) {
        mem::swap(&mut point1, &mut point2);
    }

    // wypełninie potomka -1 (puste pole)
    for x in (0..city_count) {
        // succesor.cities.push(-1);
    }

    // wstawienie miast do potomka od rodzica pomiędzy punktami cięcia,  takie zielone na slajdzie 17
    for i in (point1..=point2) {
        succesor.cities[i as usize] = parent1.cities[i as usize];
    }

    //wybranie do osobnego vectora miast ktore mogę wziąść z rodzica2 (te miasta co nie zostały pobrane z rodzica 1) z prawej storny
    let mut available_cities: Vec<i32> = Vec::new();
    for i in (point2 + 1) as usize..city_count {
        let city = parent2.cities[i];

        //wybranie do osobnego vectora miast ktore mogę wziąść z rodzica2 (te miasta co nie zostały pobrane z rodzica 1) z prawej storny
        if !succesor.cities.contains(&city)  {
            if !available_cities.contains(&city) {
                // jeśli miasta nie ma w cześci zielonej (slajd 17), dodajemy miasto do dostepnych miast
                available_cities.push(city);
            }
        }
    }



    //wybranie do osobnego vectora miast ktore mogę wziąść z rodzica2 z lewej storny
    for i in 0..=point2 {
        let city = parent2.cities[i as usize];

        // Check if the city is not in successor.cities and not in available_cities
        if !succesor.cities.contains(&city) {
            if !available_cities.contains(&city) {
                // jeśli miasta nie ma w cześci zielonej (slajd 17), dodajemy miasto do dostepnych miast
                available_cities.push(city);
            }
        }
    }

    // Wypełnienie potomka miastami z rodzica 2 częsci prawej
    for i in (point2 + 1) as usize..city_count {
        if succesor.cities[i] == -1 {
            // Assign the first city from available_cities to successor.cities[i] and remove it
            succesor.cities[i] = available_cities[0];
            available_cities.remove(0);
        }
    }

    // Wypełnienie potomka miastami z rodzica 2 częsci lewej
    for i in 0..point1 {
        if succesor.cities[i as usize] == -1 {
            // Assign the first city from available_cities to successor.cities[i] and remove it
            succesor.cities[i as usize] = available_cities[0];
            available_cities.remove(0);
        }
    }

    return succesor;
}

fn crossover(mut config: &mut Config){
    let mut succesor1 : Solution;
    let mut succesor2 : Solution;
    let upper_range = config.population.len() - 1;
    let mut range = Uniform::from(0..=upper_range as i32);
    let crossover_chance = Uniform::from(0.0 as f32 ..=1.0 as f32);
    let rng = &mut config.rng;


    let mut new_ones: Vec<Solution> = Vec::new();
    let mut population_temp = config.population.clone();

    //todo tu sie dzieje cos inczaje, shuffle raczej robi ok
    for solution in &population_temp {
        let chance: f32 = rng.sample(crossover_chance);

        if chance < config.crossover_factor {
            let rodzic1 = range.sample(rng);
            let rodzic2 = range.sample(rng);

            let parent1 = &population_temp[rodzic1 as usize];
            let parent2 = &population_temp[rodzic2 as usize];

            succesor1 = ox_crossover(parent1, parent2, rng);
            new_ones.push(succesor1);
        } else {
            let sol = solution.clone();
            new_ones.push(sol);
        }
    }


    for solution in &population_temp {
        let szansa = crossover_chance.sample(rng);

        if (szansa < config.crossover_factor) {
            let rodzic1 = range.sample(rng);
            let rodzic2 = range.sample(rng);

            let parent1 = &population_temp[rodzic2 as usize];
            let parent2 = &population_temp[rodzic1 as usize];

            succesor2 = ox_crossover(parent1, parent2, rng);
            new_ones.push(succesor2);
        } else{
            let sol = solution.clone();
            new_ones.push(sol);
        }
    }
    config.population = new_ones;
}

pub(crate) fn genetic(config: &mut Config){
    config.liczba_operacji = 0;
    generate_starting_population(config);

    config.best_solution.reset();
    let time = config.stop_time;
    let duration = Duration::from_secs(time);

    let start = Instant::now();
    let end = start + duration;


    while Instant::now() < end {
        {
            let population = &mut config.population;
            let best_solution = &mut config.best_solution;
            let matrix = &mut config.matrix;
            let rng = &mut config.rng;

            evaluate_population(population, matrix, best_solution);

            config.population = choosing_parent_book_method(population, rng);
        }
        {
            let population = &mut config.population;
            let best_solution = &mut config.best_solution;
            let matrix = &mut config.matrix;
            let rng = &mut config.rng;
            crossover(config);
        }

        let population = &mut config.population;
        let best_solution = &mut config.best_solution;
        let matrix = &mut config.matrix;
        let rng = &mut config.rng;
        evaluate_population(population, matrix, best_solution);



        mutation(config);

        config.liczba_operacji += 1;
    }




    println!("Out of time");
    println!("Liczba operacji: {}", config.liczba_operacji);
    config.print_best();
}