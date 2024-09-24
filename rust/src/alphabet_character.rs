pub fn alphabet_character (word: &str) {
	let mut result: Vec<char> = word.chars().collect();
	let length = result.len();
	for i in 0..length {
		for j in 0..length - i -1 {
			if result[j] > result [j + 1] {
				result.swap(j, j + 1);
			}
		}
	}
	println!("{:?}", result.into_iter().collect());
}