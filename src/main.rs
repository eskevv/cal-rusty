mod calculator;
mod problem_solution;
mod tests;

use calculator::Calculator;

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  let problem = "5 % 2 ^ 5 * (23 * 5 ^ 3)";

  let mut calculator = Calculator::new();


  calculator.compute(problem);
  calculator.print_solution(true, 6);
}
