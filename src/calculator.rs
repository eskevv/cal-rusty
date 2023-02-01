// BUG: parentheses not parsed well when close to each other in odd cases
// TODO: formated output using data structs and recursion / add color
// TODO: time and sound for outputting calculations
// TODO: refactor the parsing
// TODO: solve_math() faster

use crate::problem_solution::ProblemSolution;

pub struct Calculator {
  stack: i32,
  last_input: String,
  pub solution: ProblemSolution,
}

impl Calculator {
  pub fn new() -> Self {
    Calculator {
      stack: 0,
      last_input: String::new(),
      solution: ProblemSolution::new(),
    }
  }

  pub fn compute(&mut self, problem: &str) {
    if !self.valid_for_math(problem) {
      panic!("!invalid math format given");
    }

    self.last_input = problem.to_string();

    self.solution = self.parse_problem(problem, self.stack);
  }

  fn parse_problem(&mut self, problem: &str, stack: i32) -> ProblemSolution {
    let mut data = SimpliefiedParse::new(&problem);
    let mut local_solution = ProblemSolution::new();
    local_solution.stack = stack;
    local_solution.problem = problem.to_string();

    for i in problem.chars() {
      if data.index >= 1 && i == '(' && data.left_last_index == data.index - 1 {
        data.left_repeats += 1;
      }
      if data.index >= 1 && i == ')' && data.right_last_index == data.index - 1 {
        data.right_repeats += 1;
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
        self.stack += 1;

        let added = self.evaluate_parsed(problem, &mut data, local_solution.stack + 1);
        local_solution.branches.push(added);
      } else if data.index == problem.len() - 1 && data.result == problem {
        self.stack -= 1;
      }

      data.index += 1;
    }

    local_solution.answer = self.resolve_operation(&mut data.result, &mut local_solution);

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

  fn valid_for_math(&self, calculation: &str) -> bool {
    let useable = vec!['+', '-', '/', '*', '=', ' ', '(', ')'];
    calculation.chars().all(|s| s.is_ascii_digit() || useable.contains(&&s))
  }

  fn solve_math(&mut self, operators: &mut Vec<char>, numbers: &mut Vec<f32>, solution: &mut ProblemSolution) -> f32 {
    while operators.iter().find(|&&c| c == '*' || c == '/') != None {
      let presedence_1 = operators.iter().position(|&c| c == '/' || c == '*');
      if presedence_1 != None {
        let index = presedence_1.unwrap();
        let previous = numbers[index];
        match operators[index] {
          '*' => numbers[index] = numbers[index] * numbers[index + 1],
          '/' => numbers[index] = numbers[index] / numbers[index + 1],
          _ => panic!("!unhandled operator"),
        }

        solution.steps.push(format!("{} {} {} = {}", previous, operators[index], numbers[index + 1], numbers[index]));

        operators.remove(index);
        numbers.remove(index + 1);
      }
    }

    while operators.iter().find(|&&c| c == '+' || c == '-') != None {
      let presedence_1 = operators.iter().position(|&c| c == '+' || c == '-');
      if presedence_1 != None {
        let index = presedence_1.unwrap();
        let previous = numbers[index];
        match operators[index] {
          '+' => numbers[index] = numbers[index] + numbers[index + 1],
          '-' => numbers[index] = numbers[index] - numbers[index + 1],
          _ => panic!("!unhandled operator"),
        }

        solution.steps.push(format!("{} {} {} = {}", previous, operators[index], numbers[index + 1], numbers[index]));

        operators.remove(index);
        numbers.remove(index + 1);
      }
    }

    numbers[0]
  }

  fn resolve_operation(&mut self, operation: &mut str, solution: &mut ProblemSolution) -> f32 {
    let mut operators: Vec<char> = Vec::new();
    let mut numbers: Vec<f32> = Vec::new();
    let mut digit_start = 0;
    let mut digit_started = false;
    let mut index = 0;

    for c in operation.chars() {
      let mut is_operator = ['*', '-', '/', '+'].contains(&c);

      if !digit_started && (c == '-' || c.is_ascii_digit()) {
        digit_start = index;
        digit_started = true;
        is_operator = false;
      } else if is_operator {
        operators.push(c);
      }

      if digit_started && (is_operator || index == operation.len() - 1) {
        digit_started = false;
        let end = if index == operation.len() - 1 { index + 1 } else { index };
        let parsed = operation[digit_start..end].replace(' ', "").parse::<f32>();
        numbers.push(parsed.unwrap_or_default());
      }

      index += 1;
    }

    let answer = self.solve_math(&mut operators, &mut numbers, solution);
    solution.answer = answer;
    answer
  }
}

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
