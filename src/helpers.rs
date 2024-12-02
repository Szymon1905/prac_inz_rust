use std::{env, io};
use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};


// funkcje pomocnicze dla programu

// funkcja zwracająca obecną scieżkę
fn get_cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_owned()
}

// funkcja wypisująca obecną scieżkę
fn print_cwd() {
    println!("{:?}", env::current_dir());
}

// funkcja zczytujaca int do obsługi menu
pub fn read_integer() -> i32 {
    loop {
        let mut input = String::new();
        println!("Podaj int wybranej opcji:");


        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("You entered: {}", input.trim());
            }
            Err(e) => {
                println!("Bład wczytanego wyboru: {}", e);
            }
        }

        match input.trim().parse::<i32>() {
            Ok(value) => return value,
            Err(_) => println!("Nieprawdiłowa opcja."),
        }
    }
}

