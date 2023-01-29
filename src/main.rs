fn valid_for_math(calculation: &str) -> bool {
  let useable = vec!['+', '-', '/', '*', '=', ' ', '(', ')'];

  calculation
    .chars()
    .all(|s| s.is_ascii_digit() || useable.contains(&&s))
}

fn main() {
  println!("\n  ** Modern Calculator 2023 **  \n");

  let operation = "(((9 + 3) + 87)) / (((11111111 * 2)))";

  let result = math_operation(&operation.to_string(), "IIIIIIIIIIIIIIIIIIII");

  println!("\x1b[31m * [ANSWER]: \x1b[4m{result} \x1b[0m\n");
}

fn math_operation(operation: &String, flag: &str) -> f32 {
  if !valid_for_math(operation) {
    panic!("!invalid math format given");
  }

  println!("[DEBUG::ENTER]={operation}");
  let mut left_count = 0;
  let mut left_last_index = 0;
  let mut left_repeats = 0;
  let mut right_count = 0;
  let mut right_last_index = 0;
  let mut right_repeats = 0;

  let mut left_start = 0;
  let mut new_string = String::from(operation.as_str());
  let mut index = 0;
  let mut can_start = true;

  for i in operation.chars() {
    if index >= 1 && i == '(' && left_last_index == index - 1 {
      left_repeats += 1;
    }
    if index >= 1 && i == ')' && right_last_index == index - 1 {
      right_repeats += 1;
    }

    if i == '(' {
      left_count += 1;
      left_last_index = index;
    } else if i == ')' {
      right_count += 1;
      right_last_index = index;
    }

    if i == '(' && can_start {
      left_start = index;
      can_start = false;
    }

    if i == ')' && left_count == right_count {
      let repeats = std::cmp::min(left_repeats, right_repeats);
      let mut to_parse = String::from(&operation[left_start + 1 + repeats..index - repeats]);
      let to_replace = String::from(&operation[left_start..index + 1]);

      let return_recursive = math_operation(&mut to_parse, flag).to_string();
      new_string = new_string.replace(to_replace.as_str(), return_recursive.as_str());

      println!("\x1b[35m\n * [UPDATE]:\x1b[0m {}\n", { &new_string });
      println!("{flag}");
      can_start = true;
      left_repeats = 0;
      right_repeats = 0;
    }

    index += 1;
  }

  resolve_operation(&mut new_string, flag)
}

fn resolve_operation(operation: &mut String, flag: &str) -> f32 {
  let mut operators: Vec<char> = Vec::new();
  let mut numbers: Vec<f32> = Vec::new();

  let mut digit_start = 0;
  let mut digit_started = false;

  println!("\x1b[35m \n[INPUT]:\x1b[0m  {operation}\n");
  let mut flag_steps = String::new();
  for _ in 0..flag.len() - 6 {
    flag_steps += "-";
  }
  println!("<< {flag_steps} >>");

  for i in operation.char_indices() {
    let c = i.1;
    let index = i.0;

    if c == '+' || c == '-' || c == '/' || c == '*' {
      operators.push(c);
    }
    if c.is_ascii_digit() && !digit_started {
      digit_start = index;
      digit_started = true;
    }
    if digit_started && c.is_whitespace() {
      let h: String = operation[digit_start..index].chars().collect();
      let num = h.parse::<f32>().unwrap();
      numbers.push(num);
      digit_started = false;
    } else if digit_started && index == operation.len() - 1 {
      let h: String = operation[digit_start..index + 1].chars().collect();
      let num = h.parse::<f32>().unwrap();
      numbers.push(num);
      digit_started = false;
    }
  }

  println!("OPERATORS FOUND: {0:?}", operators);
  println!("NUMBERS FOUND: {0:?}", numbers);

  println!("<< {flag_steps} >>");
  let mut step = 1;
  while operators.iter().find(|&&c| c == '*' || c == '/') != None {
    let presedence_1 = operators.iter().position(|&c| c == '/' || c == '*');
    if presedence_1 != None {
      let index = presedence_1.unwrap();
      println!(
        "Step {step}: {0:?} {1} {2:?}",
        numbers[index],
        operators[index],
        numbers[index + 1]
      );
      match operators[index] {
        '*' => numbers[index] = numbers[index] * numbers[index + 1],
        '/' => numbers[index] = numbers[index] / numbers[index + 1],
        _ => panic!("!unhandled operator"),
      }
      operators.remove(index);
      numbers.remove(index + 1);
    }
    step += 1;
  }

  while operators.iter().find(|&&c| c == '+' || c == '-') != None {
    let presedence_1 = operators.iter().position(|&c| c == '+' || c == '-');
    if presedence_1 != None {
      let index = presedence_1.unwrap();
      println!(
        "Step {step}: {0:?} {1} {2:?}",
        numbers[index],
        operators[index],
        numbers[index + 1]
      );
      match operators[index] {
        '+' => numbers[index] = numbers[index] + numbers[index + 1],
        '-' => numbers[index] = numbers[index] - numbers[index + 1],
        _ => panic!("!unhandled operator"),
      }
      operators.remove(index);
      numbers.remove(index + 1);
    }
    step += 1;
  }
  println!("\x1b[36m Answer = {}\x1b[0m", numbers[0]);
  println!("<< {flag_steps} >>");

  numbers[0]
}
