pub fn slice_type() {
    let ex_str: String = String::from("Hello World");

    let first_word: &str = get_first_word(&ex_str);

    println!("First word - {first_word}");
}

fn get_first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &val) in bytes.iter().enumerate() {
        if val == b' ' {
            return &s[0..i];
        }
    }

    return s;
}
