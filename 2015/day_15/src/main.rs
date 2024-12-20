use std::str::FromStr;

use problem::{solve_main, Problem};

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

#[derive(Debug)]
struct ParseIngredientErr;
impl FromStr for Ingredient {
    type Err = ParseIngredientErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(&[':', ','][..], "");
        let split: Vec<&str> = s.split_whitespace().collect();

        let name = split[0].to_owned();
        let capacity = split[2].parse::<isize>().expect("Capacity");
        let durability = split[4].parse::<isize>().expect("durability");
        let flavor = split[6].parse::<isize>().expect("flavor");
        let texture = split[8].parse::<isize>().expect("texture");
        let calories = split[10].parse::<isize>().expect("calories");

        Ok(Self{ name, capacity, durability, flavor, texture, calories })
    }
}

impl Ingredient {
    fn from_input(input: &Vec<String>) -> Vec<Self> {
        input.iter().map(|x| Ingredient::from_str(x).unwrap()).collect()
    }

    // fn maximize_ingredients(ingredients: &Vec<Self>, teaspoons: isize, test_calorites: bool) -> usize {
    //     // Linear programming cannot be used here because the objective function is not linear for the variables,
    //     // so this is a brute forced monstrosity
    //     let mut max_score = 0;
    //     let mut i: isize = 1;
    //     while i <= teaspoons {
    //         let mut j: isize = 1;
    //         while i + j <= teaspoons {
    //             let mut pairs: Vec<(&Ingredient, isize)> = Vec::new();
    //             pairs.push((&ingredients[0], i));
    //             pairs.push((&ingredients[1], j));

    //             let score = Ingredient::objective_function(pairs, test_calorites);
    //             if score > max_score {
    //                 max_score = score;
    //             }
    //             j += 1;
    //         }
    //         i += 1;
    //     }
    //     max_score
    // }

    fn maximize_ingredients(ingredients: &Vec<Self>, teaspoons: isize, test_calorites: bool) -> usize {
        // Linear programming cannot be used here because the objective function is not linear for the variables,
        // so this is a brute forced monstrosity
        let mut max_score = 0;
        let mut i: isize = 0;
        while i <= teaspoons {
            let mut j: isize = 0;
            while i + j <= teaspoons {
                let mut k: isize = 0;
                while i + j + k <= teaspoons {
                    let mut l: isize = 0;
                    while i + j + k + l <= teaspoons {

                        if i + j + k + l != 100 {
                            l += 1;
                            continue;
                        }

                        let mut pairs: Vec<(&Ingredient, isize)> = Vec::new();
                        pairs.push((&ingredients[0], i));
                        pairs.push((&ingredients[1], j));
                        pairs.push((&ingredients[2], k));
                        pairs.push((&ingredients[3], l));

                        let score = Ingredient::objective_function(pairs, test_calorites);
                        if score > max_score {
                            max_score = score;
                        }
                        l += 1;
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        max_score
    }

    fn objective_function(ingredient_counts: Vec<(&Ingredient, isize)>, test_calories: bool) -> usize {
        let mut capacity_score = 0;
        let mut durability_score = 0;
        let mut flavor_score = 0;
        let mut texture_score = 0;
        let mut total_calories = 0;
        for (ingredient, count) in &ingredient_counts {
            capacity_score += ingredient.capacity * count;
            durability_score += ingredient.durability * count;
            flavor_score += ingredient.flavor * count;
            texture_score += ingredient.texture * count;
            total_calories += ingredient.calories * count;
        }
        if test_calories && total_calories != 500 {
            return 0;
        }
        (capacity_score.max(0) * durability_score.max(0) * flavor_score.max(0) * texture_score.max(0)) as usize
    }
}

struct Day15;

impl Problem for Day15 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let ingredients = Ingredient::from_input(input);
        Ingredient::maximize_ingredients(&ingredients, 100, false)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let ingredients = Ingredient::from_input(input);
        Ingredient::maximize_ingredients(&ingredients, 100, true)
    }
}

fn main() {
    solve_main::<Day15>();
}
