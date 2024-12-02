use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};

mod solution;
mod helpers;
mod conf;
mod mutations;
mod genetic;
mod tests;

use conf::Config;
use crate::solution::Solution;
use crate::tests::test6;


fn read_matrix(config: &mut Config) -> Vec<Vec<i32>> {

    println!("Enter the file name: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let filename = input.trim();
    println!("file: {}",filename);
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let file = File::open(filename).expect("Nie znaleziono pliku");
    let mut city_count:i32 = 0;

    let reader = BufReader::new(file);

    let mut lines = reader.lines(); // czytamy linie pliku z danymi
    if let Some(Ok(line)) = lines.next() {
        city_count = line.trim().parse().expect("Nie prawidłowy format danych (Nie liczba)");
        println!("City count: {}", city_count);
    }
    config.city_count = city_count;


    for _i in 0..city_count {
        if let Some(Ok(line)) = lines.next() {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Nie prawidłowa wartość macierzy"))
                .collect();
            matrix.push(values); // Dodajemy wartości do rzędu macierzy
        }
    }

    matrix // Zwracamy macierz
}

//funkcje ustawiające parametry

unsafe fn set_stop_time(config: &mut Config) {
    println!("Set stop time: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");


    let temp = input.trim().parse::<u64>().expect("Nie prawidłowa wartość podana");
    config.stop_time = temp;
}

unsafe fn set_starting_population_size(config: &mut Config) {
    println!("Set starting population size rate: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");


    let temp = input.trim().parse::<i32>().expect("Nie prawidłowa wartość podana");
    config.starting_population_size = temp;
}

unsafe fn set_mutation_rate(config: &mut Config) {
    println!("Set Mutation rate: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let temp = input.trim().parse::<f32>().expect("Nie prawidłowa wartość podana");
    config.mutation_rate = temp;
}



fn main() {
    let mut option:i32;
    let mut config = Config::new();
    // menu
    println!("Author: Szymon Borzdyński");
    println!("Options:  [] <- current param value");
    println!("0 - wczytaj macierz ");
    println!("1 - ustaw krytterium stopu - czas");
    println!("2 - ustaw rozmiar populacji startowej");
    println!("3 - ustaw współczynnik krzyżowania");
    println!("4 - ustaw współczynnik mutacji");
    println!("5 - ustaw metodę mutacji ");
    println!("6 - start algorytmu genetycznego");
    println!("7 - test");
    io::stdout().flush().unwrap();
    loop {
        option = helpers::read_integer();
        unsafe {
            match option {
                0 => {
                    config.matrix = read_matrix(&mut config);
                }
                1 => {
                    set_stop_time(&mut config);
                    println!("Czas: {:?}", config.stop_time);
                }
                2 => {
                    set_starting_population_size(&mut config);
                    println!("Rozmiar populacji: {:?}", config.starting_population_size);
                }
                4 => {
                    set_mutation_rate(&mut config);
                    println!("Częstopliwość mutacji: {:?}", config.mutation_rate);
                }
                5 => {
                   // todo zdecydować czy dodać alternaytwną funkcję ruletki
                }
                6 => {
                    genetic::genetic(&mut config); // uruchomienie algorytmu genetycznego
                }
                _ => println!("Nieznana opcja!"),
            }
        }
    }
}

