

use crate::conf::Config;

use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};
use crate::osobnik::Solution;


pub fn invert(solution: &mut Solution, rng: &mut Mt19937GenRand32) {
    let range = Uniform::from(0..=(solution.cities.len() - 1) as i32);

    let mut punkt1 = range.sample(rng);
    let mut punkt2 = range.sample(rng);

    if (punkt1 > punkt2) {
        (punkt1, punkt2) = (punkt2, punkt1);
    }

    let punkt1_usize = punkt1 as usize;
    let punkt2_usize = punkt2 as usize;

    let mut range_to_invert = &mut solution.cities[punkt1_usize..punkt2_usize + 1] ;
    range_to_invert.reverse();
    //range_to_invert.rev();
}

fn invertion_method(config: &mut Config) {
    let population_size = config.population.len();
    let mutation_count: i32 = (config.mutation_rate * population_size as f32) as i32; //todo is it safe ?


    // todo zoptymalziowac aby range nyl init tylko raz
    let range = Uniform::from(0 as i32..=((population_size - 1) as i32)); // todo czeck czy = zostawić w c++ mozę inaczej od właćżnie do bez
    let rng = &mut config.rng;
    for _ in 0..mutation_count {
        let mut random = range.sample(rng);
        invert(&mut config.population[random as usize], rng);
    }
}

fn swapping_method(config: &mut Config) {
    let population = &mut config.population;
    let range = Uniform::from(0..=((population.len() - 1) as i32));
    let rng = &mut config.rng;
    let punkt1 = range.sample(rng);
    let punkt2 = range.sample(rng);
    //(population[punkt1], population[punkt2]) = (population[punkt2], population[punkt1]);
    population.swap(punkt1 as usize, punkt2 as usize);
}

pub(crate) fn mutation(mut config: &mut Config) {
    if (config.mutation_method == 0) {
        invertion_method(&mut config);
    }
    if (config.mutation_method == 1) {
        swapping_method(&mut config);
    }
}

//todo test all these functions