#[cfg(test)]
use crate::calculator::Calculator;

#[test]
// #[ignore]
fn subtract_positives() {
  let input = "2-1";
  let result = Calculator::new().compute(input);
  assert_eq!(result, 1.0, "2 - 1 = 1");
}

#[test]
// #[ignore]
fn subtract_negatives() {
  let input = "-1--2";
  let result = Calculator::new().compute(input);
  assert_eq!(result, 1.0);
}

#[test]
// #[ignore]
fn start_with_negative() {
  let input = "-1-2";
  let result = Calculator::new().compute(input);
  assert_eq!(result, -3.0);
}

#[test]
// #[ignore]
fn start_subtract_negative() {
  let input = "-1--2";
  let result = Calculator::new().compute(input);
  assert_eq!(result, 1.0);
}

#[test]
// #[ignore]
fn multiple_negation() {
  let input = "-1 - --2";
  let result = Calculator::new().compute(input);
  assert_eq!(result, -3.0);
}

#[test]
// #[ignore]
fn add_negatives() {
  let input = "-1+-2";
  let result = Calculator::new().compute(input);
  assert_eq!(result, -3.0);
}

#[test]
// #[ignore]
fn braces_and_negative() {
  let input = "(-1--2)";
  let result = Calculator::new().compute(input);
  assert_eq!(result, 1.0);
}

#[test]
// #[ignore]
fn braces_three_numbers() {
  let input = "(-1--2--3)";
  let result = Calculator::new().compute(input);
  assert_eq!(result, 4.0);
}
