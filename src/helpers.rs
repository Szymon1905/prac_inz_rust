use std::{env, io};
use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};

fn get_cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_owned()
}

fn print_cwd() {
    println!("{:?}", env::current_dir());
}

pub fn read_integer() -> i32 {
    loop {
        let mut input = String::new(); // Create a new String to hold the user input
        println!("Please enter an integer:"); // Prompt the user for input

        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!"); // Handle errors while reading input

        //parse the input as an i32
        match input.trim().parse::<i32>() {
            Ok(value) => return value, // Return the parsed integer if successful
            Err(_) => println!("Invalid input. Please enter a valid integer."), // Prompt again for invalid input
        }
    }
}

