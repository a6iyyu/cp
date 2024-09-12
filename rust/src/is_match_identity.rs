use std::string::String;

pub fn is_match_identity(string: &str) -> bool {
  string == string.to_lowercase().chars().rev().collect::<String>()
}