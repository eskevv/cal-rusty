mod calculator;
mod problem_solution;
mod tests;

use calculator::Calculator;

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  let problem = "2 + 4 * 2 ^ 3 ^ 1";

  let mut calculator = Calculator::new();

  calculator.compute(problem);
  calculator.print_solution(true);
}
