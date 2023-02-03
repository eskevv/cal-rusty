// TODO: time and sound for outputting calculations
// TODO: more operators (^ %)

use crate::problem_solution::ProblemSolution;
use crate::problem_solution::Equation;

pub struct Calculator {
  last_input: String,
  results: ProblemSolution,
}

impl Calculator {
  pub fn new() -> Self {
    Calculator { last_input: String::new(), results: ProblemSolution::new() }
  }

  pub fn compute(&mut self, problem: &str) -> f32 {
    if !self.valid_for_math(problem) {
      panic!("!invalid math format given");
    }

    self.last_input = problem.to_string();

    self.results = self.parse_problem(problem, 0);
    self.results.solution.answer
  }

  pub fn print_solution(&self, compact: bool, tab_spaces: i32) {
    self.results.print_solution(compact, tab_spaces);
  }

  // [ REGION ] : parsing

  fn parse_problem(&mut self, problem: &str, stack: i32) -> ProblemSolution {
    let mut data = SimpliefiedParse::new(&problem);
    let mut branches = Vec::<ProblemSolution>::new();

    for i in problem.chars() {
      if data.index >= 1 && i == '(' && data.left_last_index == data.index - 1 && data.repeat_allowed_left {
        data.left_repeats += 1;
      } else if data.left_repeats >= 1 {
        data.repeat_allowed_left = false;
      }
      if data.index >= 1 && i == ')' && data.right_last_index == data.index - 1 && data.repeat_allowed_right {
        data.right_repeats += 1;
      } else if data.right_repeats >= 1 {
        data.repeat_allowed_right = false;
      }

      if i == '(' {
        data.left_count += 1;
        data.left_last_index = data.index;
        if data.can_start {
          data.start = data.index;
          data.can_start = false;
        }
      } else if i == ')' {
        data.right_count += 1;
        data.right_last_index = data.index;
      }

      if i == ')' && data.left_count == data.right_count {
        branches.push(self.evaluate_parsed(problem, &mut data, stack + 1));
      }

      data.index += 1;
    }

    ProblemSolution {
      stack,
      branches,
      problem: problem.to_string(),
      solution: self.resolve_operation(&data.result)
    }
  }

  fn evaluate_parsed(&mut self, operation: &str, data: &mut SimpliefiedParse, stack: i32) -> ProblemSolution {
    let repeats = std::cmp::min(data.left_repeats, data.right_repeats);
    let to_replace = &operation[data.start..data.index + 1];
    let to_calculate = &operation[data.start + 1 + repeats..data.index - repeats];
    let return_recursive: ProblemSolution = self.parse_problem(&to_calculate, stack);

    let replacement = data.result.replace(to_replace, &format!("{}", return_recursive.solution.answer));
    data.equation_replace(&replacement);

    return_recursive
  }

  // [ REGION ] : solve simplified equation

  fn solve_with(&self, lhs: f32, rhs: f32, operator: char) -> f32 {
    match operator {
      '*' => lhs * rhs,
      '/' => lhs / rhs,
      '+' => lhs + rhs,
      '-' => lhs - rhs,
      '%' => lhs % rhs,
      '^' => lhs.powf(rhs),
      _ => panic!("!unsupported operator given"),
    }
  }

  fn solve_operands(&self, equation: &mut Equation, index: usize) {
    let lhs = equation.numbers[index + 1];
    let rhs = equation.numbers[index];
    let operator = equation.operators[index];
    equation.numbers[index + 1] = self.solve_with(lhs, rhs, operator);
    equation.operators.remove(index);
    equation.numbers.remove(index); // now index instead of index + 1
    equation.steps.push(format!("{} {} {} = {}", lhs, operator, rhs, equation.numbers[index]));
  }

  fn solve_math(&mut self, equation: &mut Equation) {
    // reverse the collections before properly iterating through them in a double reverse
    // we also need to keep a counter to subtract how much the indeces shifted since the original index scan
    // solve_operands() has to be accounted for in an opposite fashion as to which index you would typically store the result in
    // ..left -> right means you store the result in left operand here we store on the right and then shift with the offset
    // all this ensures we can modify the collection with a single iterator without messing up the order of operations

    equation.operators.reverse();
    equation.numbers.reverse();

    let mut operations = 0;
    // pow operator should be default reversed
    for index in self.get_indices(&equation.operators, &['^']).iter() {
      self.solve_operands(equation, *index - operations);
      operations += 1;
    }

    for index in self.get_indices(&equation.operators, &['%']).iter().rev() {
      self.solve_operands(equation, *index);
    }
    for index in self.get_indices(&equation.operators, &['*', '/']).iter().rev() {
      self.solve_operands(equation, *index);
    }
    for index in self.get_indices(&equation.operators, &['+', '-']).iter().rev() {
      self.solve_operands(equation, *index);
    }

    equation.answer = *equation.numbers.last().expect("!this equation did not evaluate to any number correctly");
  }

  fn resolve_operation(&mut self, operation: &str) -> Equation {
    let mut members = Equation::new();
    let mut digit_start = 0;
    let mut digit_started = false;
    let mut index = 0;

    for c in operation.chars() {
      let is_operator = self.allowed_operators().contains(&c);

      if !digit_started && (c == '-' || c.is_ascii_digit()) {
        digit_start = index;
        digit_started = true;
      } else if is_operator {
        members.operators.push(c);
      }

      if digit_started && (is_operator || index == operation.len() - 1) {
        digit_started = false;
        let end = if index == operation.len() - 1 { index + 1 } else { index };
        let parsed = operation[digit_start..end].replace(' ', "").parse::<f32>();
        members.numbers.push(parsed.unwrap_or_default());
      }

      index += 1;
    }

    self.solve_math(&mut members);

    members
  }

  // [ REGION ] : utilities

  fn valid_for_math(&self, calculation: &str) -> bool {
    let mut useable = vec!['=', ' ', '(', ')'];
    useable.extend_from_slice(self.allowed_operators());
    calculation.chars().all(|s| s.is_ascii_digit() || useable.contains(&&s))
  }

  fn get_indices(&self, operators: &[char], look_for: &[char]) -> Vec<usize> {
    operators
      .iter()
      .enumerate()
      .filter(|(_, &c)| look_for.contains(&c))
      .map(|(i, _)| i)
      .collect::<Vec<usize>>()
  }

  fn allowed_operators(&self) -> &[char] {
    &['-', '+', '*', '/', '^', '%']
  }
}

// [ REGION ] : helper structs

struct SimpliefiedParse {
  result: String,
  left_count: usize,
  left_last_index: usize,
  left_repeats: usize,
  right_count: usize,
  right_last_index: usize,
  right_repeats: usize,
  repeat_allowed_left: bool,
  repeat_allowed_right: bool,
  start: usize,
  index: usize,
  can_start: bool,
}

impl SimpliefiedParse {
  fn new(result: &str) -> Self {
    Self {
      result: result.to_string(),
      left_count: 0,
      left_last_index: 0,
      left_repeats: 0,
      right_count: 0,
      right_last_index: 0,
      right_repeats: 0,
      repeat_allowed_left: true,
      repeat_allowed_right: true,
      start: 0,
      index: 0,
      can_start: true,
    }
  }

  fn equation_replace(&mut self, result: &str) {
    self.result = result.to_string();
    self.can_start = true;
    self.left_repeats = 0;
    self.right_repeats = 0;
    self.repeat_allowed_left = true;
    self.repeat_allowed_right = true;
  }
}
