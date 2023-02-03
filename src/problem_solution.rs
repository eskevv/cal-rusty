pub struct Equation {
  pub operators: Vec<char>,
  pub numbers: Vec<f64>,
  pub steps: Vec<String>,
  pub answer: f64,
}

pub struct ProblemSolution {
  pub stack: i32,
  pub problem: String,
  pub solution: Equation,
  pub branches: Vec<ProblemSolution>,
}

impl Equation {
  pub fn new() -> Self {
    Self { operators: Vec::new(), numbers: Vec::new(), steps: Vec::new(), answer: 0.0 }
  }
}

impl ProblemSolution {
  pub fn new() -> Self {
    Self {
      stack: 0,
      problem: "random".to_string(),
      branches: Vec::<ProblemSolution>::new(),
      solution: Equation::new(),
    }
  }

  fn print_branches(&self, spaces: i32, is_compact: bool, show_header: bool, tab_space: i32) {
    let mut space = String::from("");
    for _ in 0..spaces {
      space += " ";
    }

    let compact = if is_compact { "" } else { "\n" };
    let header = if show_header { format!("{}'\x1b[91mStack {}\x1b[0m'\n", space, self.stack) } else { String::new() };

    println!("{header}{}[\x1b[92m Data \x1b[0m]: {}{}", space, self.problem, compact);

    let mut index = 0;
    for i in &self.branches {
      i.print_branches((self.stack + 1) * tab_space, is_compact, index == 0, tab_space);
      index += 1;
    }

    for i in &self.solution.steps {
      println!("{}  \x1b[37m-->\x1b[0m {i}", space);
    }

    if self.stack != 0 {
      println!("{}[\x1b[93m Solution \x1b[0m]: \x1b[43m{}\x1b[0m{}", space, self.solution.answer, compact);
    } else {
      println!("{}[\x1b[36m Solution \x1b[0m]: \x1b[21m\x1b[36m{}\x1b[0m{}", space, self.solution.answer, compact);
    }
  }

  pub fn print_solution(&self, is_compact: bool, tab_space: i32) {
    println!("\nYour Input: \x1b[0m = \x1b[4m{}\x1b[0m\n", self.problem);

    self.print_branches(self.stack * tab_space, is_compact, true, tab_space);
  }
}
