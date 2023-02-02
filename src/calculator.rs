// TODO: time and sound for outputting calculations
// TODO: refactor the parsing
// TODO: more operators (^ %)

use crate::problem_solution::ProblemSolution;

pub struct Calculator {
  last_input: String,
  solution: ProblemSolution,
}

impl Calculator {
  pub fn new() -> Self {
    Calculator { last_input: String::new(), solution: ProblemSolution::new() }
  }

  pub fn compute(&mut self, problem: &str) -> f32 {
    if !self.valid_for_math(problem) {
      panic!("!invalid math format given");
    }

    self.last_input = problem.to_string();

    self.solution = self.parse_problem(problem, 0);
    self.solution.answer
  }

  pub fn print_solution(&self, compact: bool) {
    self.solution.print_solution(compact);
  }

  // [ REGION ] : parsing

  fn parse_problem(&mut self, problem: &str, stack: i32) -> ProblemSolution {
    let mut data = SimpliefiedParse::new(&problem);
    let mut local_solution = ProblemSolution::new();
    local_solution.stack = stack;
    local_solution.problem = problem.to_string();

    let mut repeat_allowed_left = true;
    let mut repeat_allowed_right = true;

    for i in problem.chars() {
      if data.index >= 1 && i == '(' && data.left_last_index == data.index - 1 && repeat_allowed_left {
        data.left_repeats += 1;
      } else if data.left_repeats >= 1 {
        repeat_allowed_left = false;
      }
      if data.index >= 1 && i == ')' && data.right_last_index == data.index - 1 && repeat_allowed_right {
        data.right_repeats += 1;
      } else if data.right_repeats >= 1 {
        repeat_allowed_right = false;
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
        let added = self.evaluate_parsed(problem, &mut data, local_solution.stack + 1);
        local_solution.branches.push(added);
        repeat_allowed_left = true;
        repeat_allowed_right = true;
      }

      data.index += 1;
    }

    self.resolve_operation(&mut data.result, &mut local_solution);

    local_solution
  }

  fn evaluate_parsed(&mut self, operation: &str, data: &mut SimpliefiedParse, stack: i32) -> ProblemSolution {
    let repeats = std::cmp::min(data.left_repeats, data.right_repeats);
    let to_replace = &operation[data.start..data.index + 1];
    let to_calculate = &operation[data.start + 1 + repeats..data.index - repeats];
    let return_recursive = self.parse_problem(&to_calculate, stack);

    data.result = data.result.replace(to_replace, &format!("{}", return_recursive.answer));
    data.can_start = true;
    data.left_repeats = 0;
    data.right_repeats = 0;

    return_recursive
  }

  // [ REGION ] : solve simplified equation

  fn solve_with(&mut self, members: &mut EquationMembers, solution: &mut ProblemSolution, index: usize) {
    let numbers = &mut members.numbers;
    let operators = &mut members.operators;

    let previous = numbers[index];
    match operators[index] {
      '*' => numbers[index] = numbers[index] * numbers[index + 1],
      '/' => numbers[index] = numbers[index] / numbers[index + 1],
      '+' => numbers[index] = numbers[index] + numbers[index + 1],
      '-' => numbers[index] = numbers[index] - numbers[index + 1],
      '^' => numbers[index] = numbers[index].powf(numbers[index + 1]),
      _ => panic!("!unhandled operator"),
    }

    solution.steps.push(format!("{} {} {} = {}", previous, operators[index], numbers[index + 1], numbers[index]));

    operators.remove(index);
    numbers.remove(index + 1);
  }

  fn solve_math(&mut self, members: &mut EquationMembers, solution: &mut ProblemSolution) -> f32 {
    // these while loops can definately be improved (too many iterators created)

    while let Some(operation_index) =  members.operators.iter().rposition(|&c| c == '^') {
      self.solve_with(members, solution, operation_index);
    }
    while let Some(operation_index) =  members.operators.iter().position(|&c| c == '*' || c == '/') {
      self.solve_with(members, solution, operation_index);
    }
    while let Some(operation_index) =  members.operators.iter().position(|&c| c == '+' || c == '-') {
      self.solve_with(members, solution, operation_index);
    }

    members.numbers[0]
  }

  fn resolve_operation(&mut self, operation: &mut str, solution: &mut ProblemSolution) {
    let mut members = EquationMembers::new();
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

    solution.answer = self.solve_math(&mut members, solution);
  }

  // [ REGION ] : utilities

  fn valid_for_math(&self, calculation: &str) -> bool {
    let mut useable = vec!['=', ' ', '(', ')'];
    useable.extend_from_slice(self.allowed_operators());
    calculation.chars().all(|s| s.is_ascii_digit() || useable.contains(&&s))
  }

  fn allowed_operators(&self) -> &[char] {
    &['-', '+', '*', '/', '^']
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
  start: usize,
  index: usize,
  can_start: bool,
}

struct EquationMembers {
  operators: Vec<char>,
  numbers: Vec<f32>,
}

impl EquationMembers {
  pub fn new() -> Self {
    Self { operators: Vec::new(), numbers: Vec::new() }
  }
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
      start: 0,
      index: 0,
      can_start: true,
    }
  }
}
