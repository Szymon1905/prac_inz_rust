/*
use rand::Rng;
use mersenne_twister::MT19937; // Import the MT19937 struct
use rand::SeedableRng; // Import the SeedableRng trait
use rand_mt::Mt64;
extern crate mersenne_twister;
extern crate rand;
use crate::conf::Config;
use crate::osobnik::Solution;

// config: Config,solution: Solution

*/
use rand::Rng;
use rand_mt::Mt19937GenRand32;

pub fn invert() {
    // Initialize the Mersenne Twister with seed 1
    let mut rng = Mt19937GenRand32::new(1);

    // Generate and print 10 uniformly distributed random numbers in a given range
    let range = 1..=10; // Change this range as needed

    for _ in 0..10 {
        let random_number: u32 = rng.gen_range(range.clone());
        println!("{}", random_number);
    }
    println!("---------");

    for _ in 0..10 {
        let random_number = 1 + rng.gen::<u32>() % 10;
        println!("{}", random_number);
    }


}

//działą
pub fn invert2() {
    // Initialize the Mersenne Twister with seed 1
    let mut rng = Mt19937GenRand32::new(1);

    // Generate and print 10 random numbers
    for _ in 0..10 {
        let random_number: u32 = rng.gen();
        println!("{}", random_number);
    }
}




























/*


pub fn invert() {
    use rand_mt::Mt64; // Import the Mersenne Twister RNG

    let mut mt = Mt64::new(1);
    for _ in 0..10 { // Generate 10 random numbers
        // Generate a random u64 number between 1 and 10
        let random_number: u64 = mt.next_u64() % 10 + 1;
        println!("{} ", random_number); // Print the generated number
    }
}



pub fn example_usage() {
    // Create a seed as a slice of u32
    let seed: [u32; 4] = [1, 2, 3, 4]; // Example seed array

    // Create a new MT19937 instance from the seed
    let mut rng = MT19937::new_unseeded(); // Seed the RNG using the array reference

    // Generate some random numbers
    for _ in 0..5 {
        let random_number: u32 = rng.gen(); // Generate a random number
        println!("Random Number: {}", random_number); // Print the generated number
    }
}

*/





fn invertion_method(){

}

fn swapping_method(){

}

fn mutation(){

}