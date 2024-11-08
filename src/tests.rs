#![allow(unused_parens)]
#![allow(unused_mut)]
use rand_core::SeedableRng;
use rand::distributions::{Distribution, Uniform};
use rand_mt::Mt19937GenRand32;
use rand::seq::SliceRandom;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;
#[test]
pub fn test1() {
    // Define seed
    let seed: u32 = 12345;

    // Initialize the Mersenne Twister engine with the seed
    let mut rng = Mt19937GenRand32::seed_from_u64(seed as u64);

    // Generate and print 5 random numbers
    for _ in 0..5 {
        println!("{}", rng.next_u32());
    }
}


pub fn test11() { // działą jak w c++
    use rand::distributions::{Distribution, Uniform};
    use rand_mt::Mt19937GenRand32;
    let seed: u32 = 12345;
    let mut rng = Mt19937GenRand32::new(seed);

    let range = Uniform::from(1..=10);

    for _ in 0..20 {
        let random_number = range.sample(&mut rng);
        println!("{}", random_number);
    }
}




use std::process;
use rand::Rng;

const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908B0DF;
const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7FFFFFFF;

struct MersenneTwister {
    mt: [u32; N],
    index: usize,
}

impl MersenneTwister {
    fn new(seed: u32) -> Self {
        let mut mt = [0u32; N];
        let mut twister = MersenneTwister { mt, index: N + 1 };
        twister.initialize(seed);
        twister.index = N;
        twister
    }

    fn initialize(&mut self, seed: u32) {
        self.mt[0] = seed;
        for i in 1..N {
            self.mt[i] = (1812433253u32)
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
    }

    fn generate_numbers(&mut self) {
        for i in 0..N {
            let y = (self.mt[i] & UPPER_MASK) | (self.mt[(i + 1) % N] & LOWER_MASK);
            self.mt[i] = self.mt[(i + M) % N] ^ (y >> 1);
            if y % 2 != 0 {
                self.mt[i] ^= MATRIX_A;
            }
        }
    }

    fn extract_number(&mut self) -> Result<u32, &'static str> {
        if self.index >= N {
            if self.index > N {
                panic!("Generator was never seeded");
            }
            self.generate_numbers();
            self.index = 0;
        }

        // Tempering Process
        let mut y = self.mt[self.index];
        self.index += 1;
        y ^= (y >> 11);
        y ^= (y << 7) & 0x9D2C5680;
        y ^= (y << 15) & 0xEFC60000;
        y ^= (y >> 18);
        Ok(y)
    }
}

#[test]
pub fn test_shuffle() {
    use rand_mt::Mt19937GenRand32;
    let seed: u32 = 2;
    let mut rng = Mt19937GenRand32::new(seed);
    let mut vector: Vec<i32> = Vec::from([1,2,3,4,5,6]);
    let rng_gen = &mut rng;
    vector.shuffle(rng_gen);
    println!("{:?}", vector);
}


#[test]
pub fn test4() {
    use rand::distributions::{Distribution, Uniform};
    use rand_mt::Mt19937GenRand32;
    // Set the seed for the RNG
    let seed: u32 = 42; // Change this to any seed you want
    let mut rng = Mt19937GenRand32::new(seed);

    // Define the range for uniform distribution
    let range = Uniform::from(0..=10); // Generates numbers between 1 and 10

    // Generate random numbers
    for _ in 0..20 {
        let random_number = range.sample(&mut rng);
        print!("{} ", random_number);
    }
}

#[test]
pub fn test5() { //działa
    let seed: u32 = 2;
    let mut rng = Mt19937GenRand32::new(seed);
    let mut vec = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    println!("Before shuffle: {:?}", vec);

    let len = vec.len();
    for i in (1..len).rev() {
        println!("przed range i = {:?}", i);
        let range = Uniform::from(0..=(i as i32));
        let j = range.sample(&mut rng);
        println!("po range j = {:?}", j);
        vec.swap(i, j as usize);               // Swap elements at indices i and j
    }
    println!("After shuffle: {:?}", vec);
}
pub fn test6() {
    let seed: u32 = 2;
    let mut rng = Mt19937GenRand32::new(seed);
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    println!("Before shuffle: {:?}", vec);

    let len = vec.len();
    for i in (1..len).rev() {
        println!("przed range i = {:?}", i);
        let range = Uniform::from(0..=i);
        let j = range.sample(&mut rng);
        println!("po range j = {:?}", j);
        vec.swap(i, j);               // Swap elements at indices i and j
    }
    println!("After shuffle: {:?}", vec);
}
#[test]
pub fn test77() {
    let seed: u32 = 2; // Change this to any seed you want
    let mut rng = Mt19937GenRand32::new(seed);
    let mut range = Uniform::from(0..=6);
    let j = range.sample(&mut rng);
    println!("{}", j);
    range = Uniform::from(0..=5);
    let j = range.sample(&mut rng);
    println!("{}", j);
    range = Uniform::from(0..=4);
    let j = range.sample(&mut rng);
    println!("{}", j);
}


