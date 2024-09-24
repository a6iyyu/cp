use std::vec::Vec;
use std::string::String;

pub fn grow_up_numbers(number: i32) -> Vec<String> {
  if number == 0 {
    return vec!["".to_string()];
  }

  (0 ..= number).rev().map(|i| i.to_string()).collect()
}