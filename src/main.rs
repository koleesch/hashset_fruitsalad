use std::collections::HashSet;

use rand::seq::SliceRandom;

fn genearte_fruit() -> &'static str {
    let fruits = [
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "imbe",
        "jackfruit",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "papaya",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "ugli",
        "vanilla",
        "watermelon",
        "ximenia",
        "yellow watermelon",
        "zucchini",
    ];

    let mut rng = rand::thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generate 100 random fruits:");

    for _ in 0..100 {
        let fruit = genearte_fruit();
        fruit_set.insert(fruit);
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
}
