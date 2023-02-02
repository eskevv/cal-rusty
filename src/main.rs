mod calculator;
mod problem_solution;
mod tests;

use calculator::Calculator;

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  let problem = "((((200 + 900 + (800 * 280) - 18898 / (700 * 2 + (200 - (270 - 900 / 2) + 900))))))";
  // BUG: adding an extra set of unnecessary parentheses over stack 0 causes errors
  // this one solves fine

  let mut calculator = Calculator::new();

  calculator.compute(problem);
  calculator.print_solution(true);
}
