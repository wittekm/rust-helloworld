extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {

        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess = String::new();

        // read_line takes a mutable reference. &guess = reference (sorta like cpp)
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        println!("It was actually: {}", secret_number);

        let guess:u32 = match guess.trim().parse() {//.ok().expect("whatever");
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => println!("You win!"),
        }
    }
}
