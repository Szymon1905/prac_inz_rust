use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};


mod osobnik;
mod helpers;
mod conf;
mod mutations;
mod genetic;

use conf::Config;

fn read_matrix(filename: &mut String) -> Vec<Vec<i32>> {

    println!("Enter the file name: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    *filename = input;

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let file = File::open("src/tsp_5.txt").expect("File not found!");
    let mut city_count:i32 = 0;

    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    if let Some(Ok(line)) = lines.next() {
        city_count = line.trim().parse().expect("Not a number!");
        println!("City count: {}", city_count);
    }

    for _i in 0..city_count {
        if let Some(Ok(line)) = lines.next() {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid matrix value"))
                .collect();
            matrix.push(values); // Assign the read values to the matrix row
        }
    }

    //println!("{:?}", matrix);
    return matrix // Return the matrix
}

unsafe fn set_stop_time(config: &mut Config) {
    println!("Set stop time: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    // Parse the input string to an i32
    let temp = input.trim().parse::<i32>().expect("Please enter a valid integer");
    config.stop_time = temp;
}

unsafe fn set_starting_population_size(config: &mut Config) {
    println!("Set starting population size rate: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    // Parse the input string to an i32
    let temp = input.trim().parse::<i32>().expect("Please enter a valid integer");
    config.starting_population_size = temp;
}

unsafe fn set_mutation_rate(config: &mut Config) {
    println!("Set Mutation rate: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    // Parse the input string to an i32
    let temp = input.trim().parse::<f32>().expect("Please enter a valid integer");
    config.mutation_rate = temp;
}



fn main() {
   println!("Author: Szymon Borzdyński");
    let mut option:i32;
    let mut filename = String::new();
    let mut config = Config::new();

    println!("Options:  [] <- current param value");
    println!("0 - read matrix");
    println!("1 - stop criteria");
    println!("2 - starting population size");
    println!("3 - mutation rate");
    println!("4 - mutation method");
    println!("5 - start genetic algorithm");

    loop {
        option = helpers::read_integer();
        unsafe {
            match option {
                0 => {
                    config.matrix = read_matrix(&mut filename);
                    println!("{:?}", config.matrix);
                }
                1 => {
                    set_stop_time(&mut config);
                    println!("{:?}", config.stop_time);
                }
                2 => {
                    set_starting_population_size(&mut config);
                    println!("{:?}", config.starting_population_size);
                }
                3 => {
                    set_mutation_rate(&mut config);
                    println!("{:?}", config.mutation_rate);
                }
                4 => {
                    //todo add code 4
                }
                5 => {
                    //todo add code 5
                }
                6 => {
                    println!("Test only");
                    mutations::invert();
                }
                _ => println!("Unknown option!"),
            }
        }
    }
}

