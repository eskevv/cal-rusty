pub struct ProblemSolution {
  pub stack: i32,
  pub problem: String,
  pub branches: Vec<ProblemSolution>,
  pub answer: f32
}

impl ProblemSolution {
  pub fn new() -> Self {
    Self { stack: 0, problem: "random".to_string(), branches: Vec::<ProblemSolution>::new(), answer: 0.0 }
  }

  fn print_branches(&self, spaces: i32) {
    let mut space = String::from("");
    for _ in 0..spaces {
      space += " ";
    }

    println!("{}Stack: {} Data: {}", space, self.stack, self.problem);

    for i in &self.branches {
      i.print_branches((self.stack + 1) * 2);
    }

    println!("{}Stack: {} \x1b[93mSolution:\x1b[0m] {}", space, self.stack, self.answer);
  }

  pub fn print_solution(&self) {
    self.print_branches(self.stack * 2);
  }
}
