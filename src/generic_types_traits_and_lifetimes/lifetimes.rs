fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetimes() {
    let string1: String = String::from("String A");
    let string2: String = String::from("String B");

    let result: &str = longest(&string1, &string2);

    println!("Longest string is {result}");
}
