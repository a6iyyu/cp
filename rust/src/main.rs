mod alphabet_character;
mod grow_up_number;
mod is_match_identity;
mod is_palindrome;
mod oop_1;

use oop_1::{Character, SexTypes};

fn main() {
    println!("{:?}", alphabet_character::alphabet_character("lamborgini")); // abgiilmnor
    println!("{:?}", alphabet_character::alphabet_character("hacker")); // acehkr
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

    let character_masculine = Character::new(
        "1",
        "Rafi Abiyyu",
        "Airlangga",
        SexTypes::Masculine,
        19,
        3,
        3,
    );

    let character_feminine = Character::new(
      "2",
      "Kusuma Rahayu",
      "Putri",
      SexTypes::Feminine,
      25,
      1,
      1
    );

    let characterr_hermaphrodite = Character::new(
      "3",
      "Alexandria",
      "Tan",
      SexTypes::Hermaphrodite,
      43,
      2,
      2,
    );

    println!("{}", character_masculine.describe_character());
    println!("{}", character_feminine.describe_character());
    println!("{}", characterr_hermaphrodite.describe_character());
}