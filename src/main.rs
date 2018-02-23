extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 0;
    let mut next_guess = "";
    let pre_low_high = "^ Your guess was";

    loop {
        if guesses == 1 {
            next_guess = "next ";
        }
        println!("Please input your {}guess.", next_guess);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                guesses = guesses + 1;
                num
            },
            Err(_) => {
                println!("stop messing around!");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} low!", pre_low_high),
            Ordering::Greater => println!("{} high!", pre_low_high),
            Ordering::Equal => {
                println!("You won in {} guesses", guesses);
                break;
            }
        }
    }
}
