const is_palindrome = (string: string) => {
	for (let i = 0; i < string.length / 2; i++) {
		return string[i] === string[string.length - 1 - i];
	}
};

console.log(is_palindrome("kakak")); // true
console.log(is_palindrome("adik")); // false
console.log(is_palindrome("sos")); // true
console.log(is_palindrome("lawak")); // false