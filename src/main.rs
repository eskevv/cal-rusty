mod calculator;
mod problem_solution;
mod tests;

use calculator::Calculator;

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  let problem = "(2 + 4 * 2 ^ 2) + (20 * 2 ^ 2)";

  let mut calculator = Calculator::new();

  calculator.compute(problem);
  calculator.print_solution(true, 6);
}
