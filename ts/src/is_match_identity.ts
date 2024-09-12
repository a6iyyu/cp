const is_match_identity = (string: String) => string === string.toLowerCase().split("").reverse().join("");

console.log(is_match_identity("aaaaaa")); // true
console.log(is_match_identity("aabbbaaa")); // false
console.log(is_match_identity("cccccc")); // false