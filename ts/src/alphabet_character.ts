const alphabet_character = (word: string) => {
  let result = word.split("");
  for (let i = 0; i < result.length; i++) {
    for (let j = 0; j < result.length - i - 1; j++) {
      if (result[j] > result[j + 1]) {
        let temp = result[j];
        result[j] = result[j + 1];
        result[j + 1] = temp;
      }
    }
  }
  console.log(result.join(""));
};

alphabet_character("lamborgini"); // abgiilmnor
alphabet_character("hacker"); // acehkr