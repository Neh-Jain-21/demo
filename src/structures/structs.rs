struct User {
    name: String,
    email: String,
    is_active: bool,
}

pub fn structs() {
    let user1: User = User {
        name: String::from("Neh Jain"),
        email: String::from("nehjain.2001@gmail.com"),
        is_active: true,
    };

    let user2: User = User {
        email: String::from("neh.jain@convegenius.ai"),
        ..user1
    };

    println!("{}", user2.name);
    println!("{}", user2.email);
    println!("{}", user2.is_active);

    struct TupleStruct(i32, i32, i32);

    let tuple: TupleStruct = TupleStruct(0, 1, 2);

    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);
}
