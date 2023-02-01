pub struct ProblemSolution {
  pub stack: i32,
  pub problem: String,
  pub answer: f32,
  pub steps: Vec<String>,
  pub branches: Vec<ProblemSolution>,
  pub tab_space: i32,
}

impl ProblemSolution {
  pub fn new() -> Self {
    Self { stack: 0, problem: "random".to_string(), branches: Vec::<ProblemSolution>::new(), answer: 0.0, steps: Vec::new(), tab_space: 8}
  }

  fn print_branches(&self, spaces: i32, is_compact: bool, show_header: bool) {
    let mut space = String::from("");
    for _ in 0..spaces {
      space += " ";
    }

    let compact = if is_compact {""} else {"\n"};
    let header = if show_header {format!("{}'\x1b[91mStack {}\x1b[0m'\n", space, self.stack)} else {String::new()};

    println!("{header}{}[\x1b[92m Data \x1b[0m]: {}{}", space, self.problem, compact);

    let mut index = 0;
    for i in &self.branches {
      i.print_branches((self.stack + 1) * self.tab_space, is_compact, index == 0);
      index += 1;
    }

    for i in &self.steps {
      println!("{}  --> {i}", space);
    }

    println!("{}[\x1b[93m Solution \x1b[0m]: \x1b[43m{}\x1b[0m{}", space, self.answer, compact);
  }

  pub fn print_solution(&self, is_compact: bool) {
    println!("\nYour Input: \x1b[0m = \x1b[4m{}\x1b[0m\n", self.problem);

    self.print_branches(self.stack * self.tab_space, is_compact, true);

    println!("\n\x1b[36m[FINAL]\x1b[0m = \x1b[4m{}\x1b[0m", self.answer);
  }
}
