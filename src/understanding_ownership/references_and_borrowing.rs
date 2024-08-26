pub fn references_and_borrowing() {
    let mut str: String = String::from("hello");
    let new_str: &str = ", world";

    append_str(&mut str, &new_str);

    println!("{str}");
}

fn append_str(s: &mut String, new_str: &str) {
    s.push_str(new_str);
}
