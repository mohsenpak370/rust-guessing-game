use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod utils {
    pub mod guess_handler;
}

use utils::guess_handler::handle_guess_result;

pub fn run_guessing_number() {
    println!("Guess the secret number between 1 and 100!");

    let sercet_number = rand::thread_rng().gen_range(1..=100);
    let mut wrong_input = false;
    let mut number_of_lives = 10;

    loop {
        match wrong_input {
            true => println!("You should choose a number."),
            false => println!("Please input your guess."),
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess {
            _ if guess.trim() == "quit" || guess.trim() == "exit" || guess.trim() == "q" => {
                println!("You quit!");
                break;
            }
            _ => (),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                wrong_input = false;
                num
            }
            Err(_) => {
                wrong_input = true;
                continue;
            }
        };
        match guess.cmp(&sercet_number) {
            Ordering::Less => {
                if handle_guess_result(Ordering::Less, &mut number_of_lives) {
                    break;
                }
            }
            Ordering::Greater => {
                if handle_guess_result(Ordering::Greater, &mut number_of_lives) {
                    break;
                }
            }
            Ordering::Equal => {
                println!("Congradulations, You won!");
                break;
            }
        }
    }
}
