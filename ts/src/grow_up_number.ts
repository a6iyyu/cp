const grow_up_number = (number: number) => {
  const result: string[] = [];
  for (let i = number; i >= 0; i--) result.push(i.toString());
	if (number === 0) return "[]";
  return result;
};

console.log(grow_up_number(5)); // ["5", "4", "3", "2", "1", "0"]
console.log(grow_up_number(3)); // ["3", "2", "1", "0"]
console.log(grow_up_number(0)); // []