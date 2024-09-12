mod grow_up_number;
mod is_match_identity;
mod is_palindrome;

fn main() {
  println!("{:?}", grow_up_number::grow_up_numbers(5)); // ["5", "4", "3", "2", "1", "0"]
  println!("{:?}", grow_up_number::grow_up_numbers(3)); // ["3", "2", "1", "0"]
  println!("{:?}", grow_up_number::grow_up_numbers(0)); // []
  println!("{:?}", is_match_identity::is_match_identity("aaaaaa")); // true
  println!("{:?}", is_match_identity::is_match_identity("aabbbaaa")); // false
  println!("{:?}", is_match_identity::is_match_identity("cccccc")); // true
  println!("{:?}", is_palindrome::is_palindrome("kakak")); // true
  println!("{:?}", is_palindrome::is_palindrome("adik")); // false
  println!("{:?}", is_palindrome::is_palindrome("sos")); // true
  println!("{:?}", is_palindrome::is_palindrome("lawak")); // false
}