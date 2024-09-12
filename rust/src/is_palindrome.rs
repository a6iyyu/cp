use std::string::String;

pub fn is_palindrome(string: &str) -> bool {
  string == string.to_lowercase().chars().rev().collect::<String>()
}