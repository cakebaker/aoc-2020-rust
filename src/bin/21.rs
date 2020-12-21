use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let foods = parse(file_content);

    println!(
        "Result of puzzle 1: {:?}",
        count_appearance_of_ingredients_without_allergens(foods)
    );
}

fn count_appearance_of_ingredients_without_allergens(
    foods: Vec<(HashSet<String>, Vec<String>)>,
) -> usize {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingredients = HashSet::new();
    let mut ingredient_appearances = Vec::new();

    for (ingredients, allergens) in foods {
        all_ingredients.extend(ingredients.clone());
        ingredient_appearances.extend(ingredients.clone());

        for allergen in allergens {
            let entry = map.entry(allergen).or_insert_with(|| ingredients.clone());
            *entry = entry.intersection(&ingredients).cloned().collect();
        }
    }

    let ingredients_without_allergens = get_ingredients_without_allergens(map, all_ingredients);
    ingredients_without_allergens
        .iter()
        .map(|ingredient| {
            ingredient_appearances
                .iter()
                .filter(|elem| *elem == ingredient)
                .count()
        })
        .sum()
}

fn get_ingredients_without_allergens(
    map: HashMap<String, HashSet<String>>,
    all_ingredients: HashSet<String>,
) -> HashSet<String> {
    let mut ingredients = all_ingredients;

    for ingredients_with_allergen in map.values() {
        for ingredient_with_allergen in ingredients_with_allergen {
            ingredients.remove(ingredient_with_allergen);
        }
    }

    ingredients
}

fn parse(file_content: String) -> Vec<(HashSet<String>, Vec<String>)> {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut foods = Vec::new();

    for line in lines {
        let items: Vec<&str> = line.split(" (contains ").collect();
        let ingredients: HashSet<String> =
            items[0].split(' ').map(|item| item.to_string()).collect();
        let allergens: Vec<String> = items[1][..(items[1].len() - 1)]
            .split(", ")
            .map(|item| item.to_string())
            .collect();

        foods.push((ingredients, allergens));
    }

    foods
}
