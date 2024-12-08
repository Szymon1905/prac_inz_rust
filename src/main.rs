use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};

mod solution;
mod helpers;
mod conf;
mod mutations;
mod genetic;

use conf::Config;
use crate::solution::Solution;



fn read_matrix(config: &mut Config) -> Vec<Vec<i32>> {

    println!("Podaj scieżkę pliku ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nie udało się zczytać danych");
    let filename = input.trim();
    println!("Plik: {}",filename);
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let file = File::open(filename).expect("Nie znaleziono pliku");
    let mut city_count:i32 = 0;

    let reader = BufReader::new(file);

    let mut lines = reader.lines(); // czytamy linie pliku z danymi
    if let Some(Ok(line)) = lines.next() {
        city_count = line.trim().parse().expect("Nieprawidłowy format danych (Nie liczba)");
        println!("Liczba miast: {}", city_count);
    }
    config.city_count = city_count;


    for _i in 0..city_count {
        if let Some(Ok(line)) = lines.next() {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Nieprawidłowa wartość macierzy"))
                .collect();
            matrix.push(values); // Dodajemy wartości do rzędu macierzy
        }
    }

    matrix // Zwracamy macierz
}

//funkcje ustawiające parametry

unsafe fn set_stop_time(config: &mut Config) {
    println!("Ustaw kryterium stopu w sekundach: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nie udało się zczytać danych");


    let temp = input.trim().parse::<u64>().expect("Nieprawidłowa wartość podana");
    config.stop_time = temp;
}

unsafe fn set_starting_population_size(config: &mut Config) {
    println!("Ustaw rozmiar populacji startowej: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nie udało się zczytać danych");


    let temp = input.trim().parse::<i32>().expect("Nieprawidłowa wartość podana");
    config.starting_population_size = temp;
}

unsafe fn set_mutation_rate(config: &mut Config) {
    println!("Podaj współczynnik mutacji: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nie udało się zczytać danych");

    let temp = input.trim().parse::<f32>().expect("Nieprawidłowa wartość podana");
    config.mutation_rate = temp;
}

unsafe fn set_crossover_rate(config: &mut Config) {
    println!("Podaj współczynnik krzyżowania: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nie udało się zczytać danych");

    let temp = input.trim().parse::<f32>().expect("Nieprawidłowa wartość podana");
    config.crossover_rate = temp;
}

unsafe fn set_mutation_method(config: &mut Config) {
    println!("Ustaw metode mutacji: ");
    println!("0 - inwersja ");
    println!("1 - swap ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nie udało się zczytać danych");

    let temp = input.trim().parse::<i32>().expect("Nieprawidłowa wartość podana");
    config.mutation_method = temp;
}



fn main() {
    let mut option:i32;
    let mut config = Config::new();
    // menu
    println!("Autor: Szymon Borzdyński");
    println!("0 - Wczytaj macierz ");
    println!("1 - Ustaw kryterium stopu - czas");
    println!("2 - Ustaw rozmiar populacji startowej");
    println!("3 - Ustaw współczynnik krzyżowania");
    println!("4 - Ustaw współczynnik mutacji");
    println!("5 - Ustaw metodę mutacji ");
    println!("6 - Start algorytmu genetycznego");
    println!("7 - Start algorytmu genetycznego z określoną liczbą pokoleń");
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
                    println!("Rozmiar populacji startowej: {:?}", config.starting_population_size);
                }
                3 => {
                    set_crossover_rate(&mut config);
                    println!("Współczynnik krzyżowania: {:?}", config.crossover_rate);
                }
                4 => {
                    set_mutation_rate(&mut config);
                    println!("Współczynnik mutacji: {:?}", config.mutation_rate);
                }
                5 => {
                   set_mutation_method(&mut config);
                }
                6 => {
                    genetic::genetic(&mut config); // uruchomienie algorytmu genetycznego
                }
                7 => {
                    println!("Podaj liczbę operacji: ");
                    let mut input: String = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Nie udało się zczytać liczby operacji");
                    let liczba_op = input.trim().parse::<u32>().expect("Nie prawidłowa wartość podana");
                    genetic::genetic_with_operation_counter(&mut config, liczba_op); // uruchomienie algorytmu genetycznego z zadaną liczbą pokoleń
                }
                _ => println!("Nieznana opcja"),
            }
        }
    }
}

