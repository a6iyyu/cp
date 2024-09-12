const is_palindrome = (string: string) => string === string.toLowerCase().split("").reverse().join("");

console.log(is_palindrome("kakak")); // true
console.log(is_palindrome("adik")); // false
console.log(is_palindrome("sos")); // true
console.log(is_palindrome("lawak")); // false