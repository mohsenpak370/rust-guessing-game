use inquire::{error::InquireError, Select};

mod guessing_number;

fn main() {
    let options: Vec<&str> = vec!["1- Guessing the secret number", "2- Gussing the secret word"];
    let selected_game: Result<&str, InquireError> = Select::new("Choose one of the following:", options).prompt();
    match selected_game {
        Ok(choice) => println!("{}! That's mine too!", choice),
        Err(_) => println!("There was an error, please try again"),
    }

    guessing_number::run_guessing_number();
}
