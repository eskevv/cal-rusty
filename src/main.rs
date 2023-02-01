mod calculator;
use calculator::Calculator;
mod tests;
mod problem_solution;

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  let problem = "(2 + 8 + (10 + 89 / (20 * 7))) * (78 * 23)";

  let mut chip = Calculator::new();

  chip.compute(problem);
  chip.solution.print_solution(true);
}
