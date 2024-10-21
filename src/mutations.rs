use std::alloc::dealloc;
use std::mem::swap;
use crate::conf::Config;
use rand::Rng;
use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};
use crate::osobnik::Solution;

const seed: u32 = 42; // todo change so reads from config

pub fn invert(solution: &mut Solution, rng: &mut Mt19937GenRand32) {

    let range = Uniform::from(0..=solution.cities.len()-1);

    let mut punkt1 = range.sample(rng);
    let mut punkt2 = range.sample(rng);

    if (punkt1 > punkt2) {
        (punkt1, punkt2) = (punkt2, punkt1);
    }


    let mut range_to_invert = &solution.cities[punkt1..punkt2 + 1];
    range_to_invert.reverse();

}


fn invertion_method(config: &mut Config){
    let population_size = config.population.len();
    let mutation_count: i32 = (config.mutation_rate * population_size as f32) as i32; //todo is it safe ?


    // todo zoptymalziowac aby range nyl init tylko raz
    let range = Uniform::from(0..=population_size-1); // todo czeck czy = zostawić w c++ mozę inaczej od właćżnie do bez
    for _ in 0..mutation_count {
        let mut random = range.sample(&mut config.rng);
        invert(&mut config.population[random], &mut config.rng);
    }

}

fn swapping_method(population: &Vec<Solution>){
    let range = Uniform::from(0..=population.len()-1);
    let mut rng = Uniform::from(seed);
    let mut punkt1 = range.sample(&mut rng);
    let mut punkt2 = range.sample(&mut rng);
    (population[punkt1], population[punkt2]) = (population[punkt2], population[punkt1]);
}

fn mutation(mut config: Config){
    if (config.mutation_method == 0) {
        invertion_method(&mut config);
    }
    if (config.mutation_method == 1) {
        swapping_method(&config.population);
    }
}