extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's play a guessing game!");
    println!("---------------------------");

    println!("I am going to think of a number.");
    println!("But why don't you decide how hard this will be");

    println!("my secret number will be between 1 and a number you pick...");
    let mut got_top_range = false;
    let mut top_range = 100;

    while !got_top_range {
        let mut top_range_input = String::new();
        io::stdin().read_line(&mut top_range_input)
            .expect("Failed to read line");

        top_range = match top_range_input.trim().parse() {
            Ok(num) => {
                got_top_range = true;
                num
            },
            Err(_) => {
                println!("stop goofing - how aboout a number now?!");
                continue;
            },
        };
    }
    guessin_time(&top_range);
}

fn guessin_time(top_range: &u32) {
    println!("Ok, got it! I'm thinking of a number between 1 and {}:", top_range);
    println!("How many guesses will it take for you to figure it out?");

    let top_range = top_range + 1;
    let secret_number = rand::thread_rng().gen_range(1, top_range);
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
