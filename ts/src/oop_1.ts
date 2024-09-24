interface SexTypes {
  type: "Feminime" | "Masculine" | "Hermaphrodite";
}

interface Physique {
  body_types: [
    ["Emaciated", "Thin", "Lean", "Toned", "Defined", "Sculpted", "Ripped"],
    ["Gaunt", "Slender", "Slim", "Fit", "Taut", "Cut", "Shredded"],
    ["Underweight", "Svelte", "Lithe", "Athletic", "Muscular", "Built", "Solid"],
    ["Average", "Wiry", "Balanced", "Stocky", "Husky", "Buff", "Hefty"],
    ["Soft", "Slightly Firm", "Firm", "Bulky", "Strong", "Burly", "Massive"],
    ["Pudgy", "Chubby", "Stout", "Robust", "Thick", "Powerhouse", "Brawny"],
    ["Overweight", "Portly", "Heavyset", "Large", "Hulking", "Gargantuan", "Herculean"],
  ];
}

interface ICharacterData {
  id: string;
  forename: string;
  surname: string;
  sex_types: SexTypes["type"];
  age: number;
  fat: number;
  muscle: number;
  physique: string;
}

class Character implements ICharacterData {
  id: string;
  forename: string;
  surname: string;
  sex_types: SexTypes["type"];
  age: number;
  fat: number;
  muscle: number;
  physique: string;

  constructor(id: string, forename: string, surename: string, sex_types: SexTypes["type"], age: number, fat: number, muscle: number) {
    this.id = id;
    this.forename = forename;
    this.surname = surename;
    this.sex_types = sex_types;
    this.age = age;
    this.fat = this.ValidateRange(fat, 0, 3);
    this.muscle = this.ValidateRange(muscle, 0, 3);
    this.physique = this.CalculatePhysique();
  }

  private ValidateRange(value: number, min: number, max: number): number {
    return Math.max(min, Math.min(max, value));
  }

  private CalculatePhysique(): string {
    const body_types: Physique["body_types"] = [
      ["Emaciated", "Thin", "Lean", "Toned", "Defined", "Sculpted", "Ripped"],
      ["Gaunt", "Slender", "Slim", "Fit", "Taut", "Cut", "Shredded"],
      ["Underweight", "Svelte", "Lithe", "Athletic", "Muscular", "Built", "Solid"],
      ["Average", "Wiry", "Balanced", "Stocky", "Husky", "Buff", "Hefty"],
      ["Soft", "Slightly Firm", "Firm", "Bulky", "Strong", "Burly", "Massive"],
      ["Pudgy", "Chubby", "Stout", "Robust", "Thick", "Powerhouse", "Brawny"],
      ["Overweight", "Portly", "Heavyset", "Large", "Hulking", "Gargantuan", "Herculean"],
    ]

    return body_types[this.fat][this.muscle];
  }

  DescribeCharacter(): string {
    const gender_description = this.sex_types === "Feminime" ? (this.age <= 21 ? "Girl" : "Woman") : this.sex_types === "Masculine" ? (this.age <= 21 ? "Boy" : "Man") : "Hermaphrodite";
    return `A ${this.physique} ${gender_description}`;
  }
}

console.log(new Character("1", "Alex", "Graham", "Masculine", 32, 4, 50));