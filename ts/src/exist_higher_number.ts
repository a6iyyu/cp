const exist_higher_number = (list: number[], number: number) => {
  for (let i = 0; i < list.length; i++) if (list[i] > number) return true;
  return false;
};

console.log(exist_higher_number([8, 4, 20, 32, 1], 10)); // true
console.log(exist_higher_number([1, 3, 7, 4, 6], 8)); // false
console.log(exist_higher_number([], 1)); // false