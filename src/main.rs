mod calculator;
mod problem_solution;
mod tests;

use calculator::Calculator;

fn main() {
  println!("\n ** Rusty-Cal 23 ** \n");

  let h = vec![2, 2, 2, 5];

  for i in h.iter().rev() {
    println!("{i} : {}", i);
  }

  let problem = "2 ^ 2 ^ 3 + 7 ^ 2";
  // BUG: adding an extra set of unnecessary parentheses over stack 0 causes errors
  // this one solves fine

  let mut calculator = Calculator::new();

  calculator.compute(problem);
  calculator.print_solution(true);
}
