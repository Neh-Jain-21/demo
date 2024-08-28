fn with_macro() {
    #[derive(Debug)]
    struct User {
        id: i32,
        name: String,
    }

    let v: Vec<User> = vec![
        User {
            id: 1,
            name: String::from("Neh 1"),
        },
        User {
            id: 2,
            name: String::from("Neh 2"),
        },
        User {
            id: 3,
            name: String::from("Neh 3"),
        },
    ];

    let third: &User = &v[2];

    println!("The third element is {} - {}", third.id, third.name);
}

fn with_struct() {
    let mut v_new: Vec<i32> = Vec::new();

    v_new.push(1);
    v_new.push(2);
    v_new.push(3);

    let third: Option<&i32> = v_new.get(4);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn with_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        match i {
            SpreadsheetCell::Int(num) => println!("{}", num),
            SpreadsheetCell::Float(num) => println!("{}", num),
            SpreadsheetCell::Text(string) => println!("{}", string),
        }
    }
}

pub fn vectors() {
    with_macro();
    with_struct();
    with_enum();
}
