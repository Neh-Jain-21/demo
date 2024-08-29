use std::collections::HashMap;

pub fn hash_maps() {
    let mut numbers: HashMap<i32, String> = HashMap::new();

    numbers.insert(1, String::from("One"));
    numbers.insert(2, String::from("Two"));

    // Adding a Key and Value Only If a Key Isn’t Present
    numbers.entry(3).or_insert(String::from("Three"));

    // Default value if key is not present
    let binding: String = String::from("One");

    // Accessing a value in hashmap
    let number_one: &String = numbers.get(&1).clone().unwrap_or(&binding);

    println!("{number_one}");
}
