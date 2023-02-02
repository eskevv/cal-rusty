mod calculator;
mod problem_solution;
mod tests;

use calculator::Calculator;

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  let problem = "(2 ^ 2 ^ 3 + 7 ^ 2) ^ 3 + (- 87 + 56 / (40 * 2))";

  let mut calculator = Calculator::new();

  calculator.compute(problem);
  calculator.print_solution(true);
}
