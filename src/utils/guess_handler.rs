use std::cmp::Ordering;

pub fn handle_guess_result(ordering: Ordering, number_of_lives: &mut u32) -> bool {
  *number_of_lives -= 1;
  match number_of_lives {
      0 => {
          println!("Sorry, You lost!");
          true
      }
      1 => {
          let message = match ordering {
              Ordering::Less => "Too small! It's your last chance.",
              Ordering::Greater => "Too big! It's your last chance.",
              _ => unreachable!(),
          };
          println!("{message}");
          false
      }
      _ => {
          let message = match ordering {
              Ordering::Less => format!("Too small! You have {number_of_lives} more chances."),
              Ordering::Greater => format!("Too big! You have {number_of_lives} more chances."),
              _ => unreachable!(),
          };
          println!("{message}");
          false
      }
  }
}