#![allow(unused_parens)]
#![allow(unused_mut)]
use rand_mt::Mt19937GenRand32; // Assuming you're using this RNG
use rand_core::SeedableRng;

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

pub fn test3() { // działą jak w c++
    use rand::distributions::{Distribution, Uniform};
    use rand_mt::Mt19937GenRand32;
    let seed: u32 = 42;
    let mut rng = Mt19937GenRand32::new(seed);

    let range = Uniform::from(1..=10);

    for _ in 0..5 {
        let random_number = range.sample(&mut rng);
        println!("{}", random_number);
    }
}




use std::process;

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

pub fn test2() {
    let pid = process::id() as u32;
    let mut rng = MersenneTwister::new(12345);
    for i in 0..20 {
        match rng.extract_number() {
            Ok(num) => println!("PRNG {} = {}", i+1, num),
            Err(e) => eprintln!("Error:{}", e),
        }
    }
}
