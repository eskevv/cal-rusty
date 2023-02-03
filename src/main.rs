use std::io::Write;

mod calculator;
mod problem_solution;
mod tests;

use calculator::Calculator;

fn flush_message(message: &str) {
  print!("{message}");
  std::io::stdout().flush().expect("!failed to flush the stdout buffer");
}

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  flush_message("Would you like to enter your own input? (y/n) ");
  let mut option = String::new();
  std::io::stdin().read_line(&mut option).expect("!failed to read line");

  let problem = if option.trim() == "y" {
    flush_message("Enter what you want to compute: ");
    let mut string_given = String::new();
    std::io::stdin().read_line(&mut string_given).expect("!failed to read line");
    String::from(string_given.trim())
  } else {
    String::from("4 * -2".trim())
  };


  let mut calculator = Calculator::new();

  calculator.compute(&problem);
  calculator.print_solution(true, 6);
}
