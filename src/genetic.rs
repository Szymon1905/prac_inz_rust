use std::mem;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::time::{Instant, Duration};
use crate::conf::Config;
use crate::solution::Solution;
use crate::mutations::mutation;
use rand_mt::Mt19937GenRand32;

//Metoda wypisująca najlepsze znalezione rozwiązanie
fn print_best(mut config: Config){
    println!("Najlepsze rozwiązanie {:?}", config.best_solution.cities);
}


//Metoda implementująca tasowanie fisher yates
pub fn fisher_yates_shuffle(vec: &mut Vec<i32>, rng: &mut Mt19937GenRand32) {
    let len = vec.len();
    for i in (1..len).rev() {
        let range = Uniform::from(0..=i as i32);
        let j = range.sample(rng);
        vec.swap(i, j as usize);
    }
}

// Metoda generująca populację startową
fn generate_starting_population(config: &mut Config){
    // generate cities / cities
    let mut cities: Vec<i32> = Vec::new();
    let city_count = config.city_count;
    println!("Liczba miast: {}", city_count);
    for i in (0..city_count) {
        cities.push(i);
    }

    // zerujemy populację po restarcie
    config.population.clear();

    let rng = &mut config.rng;
    let starting_population_size = config.starting_population_size;
    // generujemy losowe permutacje miast do szuflowania i wstawiamy do populacji startowej
    for i in (0..starting_population_size) {
        fisher_yates_shuffle(&mut cities, rng);
        let solution = Solution::new(cities.clone(), i32::MAX);
        config.population.push(solution);
    }

    for mut solution in config.population.iter_mut(){
        let sol = solution;
        let mat = config.matrix.clone();
        sol.path_length = calculate_path_length(sol, mat);
    }
}

// Metoda obliczająca długość scieżki danego osobnika
pub(crate) fn calculate_path_length(solution: &mut Solution, matrix: Vec<Vec<i32>>) -> i32 {
    let mut sum_path: i32 = 0;
    let cities_count = solution.cities.len() - 1;


    for i in (0..cities_count){
        sum_path = sum_path + matrix[solution.cities[i] as usize][solution.cities[i + 1] as usize];
    }
    // wracamy do pierwszego miasta
    sum_path += matrix[solution.cities[cities_count] as usize][solution.cities[0] as usize];
    return sum_path;
}

// Metoda oceniajaca całą populację
fn evaluate_population(population: &mut Vec<Solution>, global_matrix: &Vec<Vec<i32>>, best_solution: &mut Solution) {
    for solution in population.iter_mut() {
        // obliczenie całkowitej drogi
        solution.path_length = calculate_path_length(solution, global_matrix.clone());

        // porównanie z obecnie najlepszym rozwiązaniem
        if solution.path_length < best_solution.path_length {
            // Aktualizacja najlepszego rozwiązania dla przeżywalności najlepszego osobnika
            *best_solution = solution.clone();

            // Wypisanie nowego najlepszego
            println!("Nowy najlepszy: {}", solution.path_length);
        }
    }
}

// Metoda implementująca ruletkę, wybierjąca osobniki do krzyżówania
fn choosing_parent_book_method(population: &[Solution], rng : &mut Mt19937GenRand32) -> Vec<Solution> {
    // Obliczenie sumy odwrotności
    let path_sum: f32 = population
        .iter()
        .map(|solution| 1.0f32 / solution.path_length as f32)
        .sum();

    // Inicjalizacja pustego vectora na wybrane osobniki
    let mut chosen_ones: Vec<Solution> = Vec::new();


    let distribution = Uniform::from(0.0..path_sum); // new -> from

    // Wybieranie rodziców
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

    chosen_ones //zwracamy wybranych
}

// funkcja sprawdzająca czy dane miasto jest w vectorze
fn check_if_contains(vector: &[i32], liczba: i32) -> bool {
    for &pole in vector {
        if pole == liczba {
            return true;
        }
    }
    return false;
}

// Metoda implementująca krzyżównaie OX 2 rodziców w celu stworzenia potomka
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
            succesor.cities[i] = available_cities[0];
            available_cities.remove(0);
        }
    }

    // Wypełnienie potomka miastami z rodzica 2 częsci lewej
    for i in 0..point1 {
        if succesor.cities[i as usize] == -1 {
            succesor.cities[i as usize] = available_cities[0];
            available_cities.remove(0);
        }
    }

    return succesor;
}

// Funkcja przygotowująca krzyżówanie, zwracająca nową populację
fn crossover(mut config: &mut Config){
    let mut succesor1 : Solution;
    let mut succesor2 : Solution;
    let upper_range = config.population.len() - 1;
    let mut range = Uniform::from(0..=upper_range as i32); // czy = rowna nie psuje, raczej nie
    let crossover_chance = Uniform::from(0.0 as f32 ..1.0 as f32);
    let rng = &mut config.rng;


    let mut new_ones: Vec<Solution> = Vec::new();
    let mut population_temp = config.population.clone();


    for solution in &population_temp {
        let chance: f32 = rng.sample(crossover_chance);

        if chance < config.crossover_rate {
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

        if (szansa < config.crossover_rate) {
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

// Główna funkcja alogrytmu genetycznego używająca określonego czasu jako warunku stopu
pub(crate) fn genetic(config: &mut Config){
    config.operation_count = 0;
    generate_starting_population(config);

    config.best_solution.reset();
    let time = config.stop_time;
    let duration = Duration::from_secs(time);

    let start = Instant::now();
    let end = start + duration;

    {
        let population = &mut config.population;
        let best_solution = &mut config.best_solution;
        let matrix = &mut config.matrix;
        evaluate_population(population, matrix, best_solution);
    }


    while Instant::now() < end {
        {
            let population = &mut config.population;
            let best_solution = &mut config.best_solution;
            let matrix = &mut config.matrix;
            evaluate_population(population, matrix, best_solution);
        }

        {
            let population = &mut config.population;
            let rng = &mut config.rng;
            config.population = choosing_parent_book_method(population, rng);
        }



        {
            crossover(config);
        }

        {
            let population = &mut config.population;
            let best_solution = &mut config.best_solution;
            let matrix = &mut config.matrix;
            evaluate_population(population, matrix, best_solution);

        }


        mutation(config);

        config.operation_count += 1;
    }




    println!("Koniec czasu !");
    println!("Wykonana liczba pokoleń: {}", config.operation_count);
    config.print_best();
}

// Główna funkcja algorytmu genetycznego używająca określonej liczby pokoleń jako warunku stopu uzyta do testów wydajności
pub(crate) fn genetic_with_operation_counter(config: &mut Config, operation_count: u32){
    let mut op_count = operation_count.clone();
    generate_starting_population(config);

    config.best_solution.reset();

    let start = Instant::now();

    {
        let population = &mut config.population;
        let best_solution = &mut config.best_solution;
        let matrix = &mut config.matrix;
        evaluate_population(population, matrix, best_solution);
    }


    while op_count > 0 {
        {
            let population = &mut config.population;
            let best_solution = &mut config.best_solution;
            let matrix = &mut config.matrix;
            evaluate_population(population, matrix, best_solution);

        }

        {
            let population = &mut config.population;
            let rng = &mut config.rng;
            config.population = choosing_parent_book_method(population, rng);
        }

        {
            crossover(config);
        }

        mutation(config);

        op_count -= 1;
    }

    let duration = start.elapsed();

    println!("Zadana liczba operacji ukończona");
    println!("Czas w milisekundach: {} ms", duration.as_millis());
    config.print_best();
}