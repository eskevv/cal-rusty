mod calculator;
use calculator::Calculator;
mod tests;

fn main() {
  println!("\n ** Modern Calculator 2023 ** \n");

  let problem = "(100 + ( (20 - 90) + (100 - 11) ) - (87 -110))";

  let mut chip = Calculator::new();
  let answer = chip.compute(problem);

  chip.print_steps();
}
