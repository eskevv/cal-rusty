// use core::panic;
// use std::num;
// // use core::{num::dec2flt::parse, panic};
// // fn get_input() -> String {
// //   print!("Enter an operation: ");
// //   io::stdout().flush().unwrap();

// //   let mut user_input = String::new();
// //   io::stdin().read_line(&mut user_input).unwrap();
// //   user_input.chars().filter(|c| !c.is_whitespace()).collect()
// // }

// // fn parse_precedence(value: &str) -> Vec<i32> {
// //   let mut operations: Vec<i32> = Vec::new();

// //   let mut left_counter = 0;
// //   let mut right_counter = 0;

// //   for c in value.chars() {
// //     if c == '(' {
// //       left_counter += 1;
// //     }
// //     if c == ')' && right_counter + 1 == left_counter {
// //       right_counter += 1;
// //       let end_index = value.find(c).unwrap();
// //       parse_precedence(&value[0..end_index - 1]);
// //     }
// //   }

// //   operations
// // }

// // fn convert_to_operation(value: &String) {
// //   if value.chars().any(|c| !valid_for_math(c)) {
// //     panic!("!one or more invalid characters for math operations");
// //   }

// //   if !value.chars().any(|c| c == '+') {
// //     panic!("!no operations passed");
// //   }

// //   if value.ends_with('+') {
// //     panic!("!invalid format");
// //   }

// //   let numbers: Vec<&str> = value.split('+').collect();

// //   let num_1: i32 = numbers[0].parse().unwrap();
// //   let num_2: i32 = numbers[1].parse().unwrap();

// //   println!("{num_1} + {num_2} = {:?}", num_1 + num_2);
// // }

// // fn compute(value: &str) -> f32 {
// //   let mut collection: Vec<&str> = Vec::new();

// //   let mut left_math = "";
// //   let right_math = "";

// //   let mut left_counter = 0;
// //   let mut right_counter = 0;
// //   let mut start = 0;
// //   let mut index = 0;

// //   for c in value.chars() {
// //     left_counter += if c == '(' { 1 } else { 0 };
// //     right_counter += if c == ')' { 1 } else { 0 };

// //     if c == ')' && right_counter == left_counter {

// //       return compute(&value[start + 1..index]);
// //     }

// //     if c == '+' && right_counter == left_counter {
// //       start = value[index..].find('(').unwrap() + index;
// //     }

// //     index += 1;
// //   }

// //   // collection
// // }
// // fn parse_operation(value: &str) -> i32 {
// //   if value.chars().any(|c| !valid_for_math(c)) {
// //     panic!("!invalid");
// //   }

// //   let left_index = value.find(|c: char| c.is_ascii_digit()).unwrap();
// //   let slice_mid = &value[left_index + 1..];
// //   let right_index = slice_mid.find(|c: char| c.is_ascii_digit()).unwrap() + (left_index + 1) + 1;
// //   let operation = &value[left_index..right_index];

// //   let valid_operators = ['+', '-', '=', '*'];

// //   println!("{operation}");

// //   let k = operation
// //     .find(|c: char| valid_operators.contains(&c))
// //     .unwrap();

// //   let operator = &operation[k..k];
// //   let first_num = &operation[left_index..left_index + 1];
// //   let second_num = &operation[right_index..right_index + 1];

// //   println!("{first_num} {operator} {second_num}");

// //   0
// // }

// fn valid_for_math(c: char) -> bool {
//   vec!['+', '-', '/', '*', '=', ' '].contains(&c) || c.is_ascii_digit()
// }

// fn basic_operation(operation: &str) -> f32 {
//   if operation.chars().any(|c| !valid_for_math(c)) {
//     panic!("!invalid operational format");
//   }

//   let valid_operators = ['+', '-', '/', '*'];
//   let operators_found: Vec<char> = operation
//     .chars()
//     .filter(|c| valid_operators.contains(c))
//     .collect();

//   if operators_found.len() != 1 {
//     panic!("!invalid operational format");
//   }

//   let operands: Vec<&str> = operation.split(operators_found[0]).collect();
//   let operand_one: f32 = operands[0]
//     .trim()
//     .parse()
//     .expect("!digits should not contain spaces");
//   let operand_two: f32 = operands[1]
//     .trim()
//     .parse()
//     .expect("!digits should not contain spaces");

//   match operators_found[0] {
//     '-' => operand_one - operand_two,
//     '+' => operand_one + operand_two,
//     '*' => operand_one * operand_two,
//     '/' => operand_one / operand_two,
//     _ => panic!("!unhandled operation"),
//   }
// }

fn math_operation(operation: &mut String) -> f32 {
  println!("[ENTER]  {operation}");
  let mut operators: Vec<char> = Vec::new();
  let mut numbers: Vec<f32> = Vec::new();

  let mut digit_start = 0;
  let mut digit_started = false;

  let mut left = 0;
  let mut right = 0;
  let mut left_start = 0;

  let mut new_string = String::from(operation.as_str());
  let mut index = 0;

  for i in operation.chars() {
    left += if i == '(' { 1 } else { 0 };
    right += if i == ')' { 1 } else { 0 };

    if i == '(' && left >= right {
      left_start = index;
    }

    if i == ')' && left == right {
      let mut to_parse = String::from(&operation[left_start + 1..index]);
      let to_replace = String::from(&operation[left_start..index + 1]);
      let return_recursive = math_operation(&mut to_parse).to_string();
      new_string = new_string.replace(to_replace.as_str(), return_recursive.as_str());
      println!("[UPDATE]: {}", { &new_string });
    }

    index += 1;
  }

  println!("[EXIT]   {new_string}");

  for i in new_string.char_indices() {
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
      let h: String = new_string[digit_start..index].chars().collect();
      let num = h.parse::<f32>().unwrap();
      numbers.push(num);
      digit_started = false;
    } else if digit_started && index == new_string.len() - 1 {
      let h: String = new_string[digit_start..index + 1].chars().collect();
      let num = h.parse::<f32>().unwrap();
      numbers.push(num);
      digit_started = false;
    }
  }

  println!("\nOPERATORS FOUND: {0:?}", operators);
  println!("NUMBERS FOUND: {0:?}\n", numbers);

  while operators.iter().find(|&&c| c == '*' || c == '/') != None {
    let presedence_1 = operators.iter().position(|&c| c == '/' || c == '*');
    if presedence_1 != None {
      let index = presedence_1.unwrap();
      println!("Before: {0:?} {1} {2:?}", numbers[index], operators[index], numbers[index + 1]);
      match operators[index] {
        '*' => numbers[index] = numbers[index] * numbers[index + 1],
        '/' => numbers[index] = numbers[index] / numbers[index + 1],
        _ => panic!("!unhandled operator"),
      }
      operators.remove(index);
      numbers.remove(index + 1);
      println!("After: {:?}\n", numbers[index]);
    }
  }
  while operators.iter().find(|&&c| c == '+' || c == '-') != None {
    let presedence_1 = operators.iter().position(|&c| c == '+' || c == '-');
    if presedence_1 != None {
      let index = presedence_1.unwrap();
      println!("Before: {0:?} {1} {2:?}", numbers[index], operators[index], numbers[index + 1]);
      match operators[index] {
        '+' => numbers[index] = numbers[index] + numbers[index + 1],
        '-' => numbers[index] = numbers[index] - numbers[index + 1],
        _ => panic!("!unhandled operator"),
      }
      operators.remove(index);
      numbers.remove(index + 1);
      println!("After: {:?}\n", numbers[index]);
    }
  }

  // println!("{:?}", numbers[0]);
  numbers[0]
}

// fn replace_str(value: &mut str, with: &mut str, start: usize) {
//   let mut index: usize = 0;
//   for i in value.chars() {
//     if index < start {
//       continue;
//     }

//     value.replace = with.chars().nth(index).unwrap();

//     index += 1;
//     if index == value.len() {
//       break;
//     }
//   }
// }

// fn try_fn(words: &mut String) {
//   let mut new_words = words.clone();
//   for c in new_words.chars_mut() {
//     if c == 'a' {
//       *c = 'i';
//     }
//   }
//   *words = new_words;
// }

// fn try_fn(words: &mut str) {
//   let wora = words.replace("a", "a").as_str();

//   *words = wora;
// }

// fn try_fn(words: &mut String) {
//   let new_string = words.replace("a", "i");
//   *words = new_string;
// }

fn main() {
  println!("\nModern Calculator 2023");

  // let operation = "(2 + 2) + ((3 + 1) + (4 + 8 / 2))";
  let operation = "(2 + 9982 / 43 + 7) / (90 + 12 / 91 + 78 / 787 * 81) / (43 + 899)";

  math_operation(&mut operation.to_string());

  // println!("{operation} = {:?}", basic_operation(operation));
}
