use std::{env, io};
use rand_mt::Mt19937GenRand32;
use rand::distributions::{Distribution, Uniform};


// funkcje pomocnicze dla programu

// funkcja zczytujaca int do obsługi menu
pub fn read_integer() -> i32 {
    loop {
        let mut input = String::new();
        println!("Podaj int wybranej opcji:");


        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Wczytano: {}", input.trim());
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

