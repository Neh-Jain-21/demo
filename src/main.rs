fn main() {
    enums();

    // struct_method_syntax();
    // structs();

    // slice_type();
    // references_and_borrowing();
    // ownership();

    // control_flows();
    // functions();
    // data_types();
    // variables_and_mutability();
}

fn enums() {
    #[derive(Debug)]
    enum Message {
        Text(String),
        Color(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Self::Text(string) => println!("{}", string),
                Self::Color(r, g, b) => println!("rgb({r}, {g}, {b})"),
            }
        }
    }

    Message::Text(String::from("Hello World")).call();
    Message::Color(0, 0, 0).call();
}

// fn struct_method_syntax() {
//     struct Rectangle {
//         width: i32,
//         height: i32,
//     }

//     impl Rectangle {
//         fn area(&self) -> i32 {
//             self.width * self.height
//         }

//         fn can_hold(&self, other: &Rectangle) -> bool {
//             self.width > other.width && self.height > other.height
//         }
//     }

//     let rectangle1: Rectangle = Rectangle {
//         width: 20,
//         height: 20,
//     };

//     let rectangle2: Rectangle = Rectangle {
//         width: 30,
//         height: 30,
//     };

//     println!("Area of Rectangle 1: {}", rectangle1.area());
//     println!("Area of Rectangle 2: {}", rectangle2.area());

//     println!("Can 1 hold 2: {}", rectangle1.can_hold(&rectangle2));
//     println!("Can 2 hold 1: {}", rectangle2.can_hold(&rectangle1));
// }

// fn structs() {
//     struct User {
//         name: String,
//         email: String,
//         is_active: bool,
//     }

//     let user1: User = User {
//         name: String::from("Neh Jain"),
//         email: String::from("nehjain.2001@gmail.com"),
//         is_active: true,
//     };

//     let user2: User = User {
//         email: String::from("neh.jain@convegenius.ai"),
//         ..user1
//     };

//     println!("{}", user2.name);
//     println!("{}", user2.email);
//     println!("{}", user2.is_active);

//     struct TupleStruct(i32, i32, i32);

//     let tuple: TupleStruct = TupleStruct(0, 1, 2);

//     println!("{}", tuple.0);
//     println!("{}", tuple.1);
//     println!("{}", tuple.2);
// }

// fn slice_type() {
//     let ex_str: String = String::from("Hello World");

//     let first_word: &str = get_first_word(&ex_str);

//     println!("First word - {first_word}");

//     fn get_first_word(s: &str) -> &str {
//         let bytes: &[u8] = s.as_bytes();

//         for (i, &val) in bytes.iter().enumerate() {
//             if val == b' ' {
//                 return &s[0..i];
//             }
//         }

//         return s;
//     }
// }

// fn references_and_borrowing() {
//     let mut str: String = String::from("hello");
//     let new_str: &str = ", world";

//     append_str(&mut str, &new_str);

//     println!("{str}");

//     fn append_str(s: &mut String, new_str: &str) {
//         s.push_str(new_str);
//     }
// }

// fn ownership() {
//     let s: String = String::from("hello");

//     takes_ownership(s);

//     let x: i32 = 5;

//     makes_copy(x);

//     fn takes_ownership(some_string: String) {
//         println!("{some_string}");
//     }

//     fn makes_copy(some_integer: i32) {
//         println!("{some_integer}");
//     }
// }

// fn control_flows() {
//     let number: i32 = 5;

//     if number == 5 {
//         println!("Number is 5");
//     } else if number >= 0 {
//         print!("Number is positive")
//     } else {
//         println!("Number is negative");
//     }

//     // Short hand if else
//     let condition: bool = true;
//     let result: i32 = if condition { 1 } else { 0 };

//     println!("The number is {result}");

//     let mut counter: i32 = 0;

//     let counter_result: i32 = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("Counter value is {counter_result}");

//     while counter != 0 {
//         println!("{counter}");

//         counter -= 1;
//     }

//     println!("While loop over!");

//     let a: [i32; 5] = [1, 2, 3, 4, 5];

//     for element in a {
//         println!("For loop count {element}");
//     }

//     println!("For loop over");
// }

// fn functions() {
//     let x: i32 = five();
//     println!("The value of x is: {x}");

//     fn five() -> i32 {
//         return 5;
//     }
// }

// fn data_types() {
//     const _SIGNED_INTEGER: usize = 5;
//     let _unsigned_integer: i32 = -10;
//     let _floating_point: f64 = 10.0;
//     let _character: char = 'a';
//     let _boolean: bool = true;
//     let _string: &str = "Hello, world!";

//     let tuple: (i32, &str) = (10, "Hello, world!");
//     let tuple_second: &str = tuple.1;
//     println!("The value of tuple at index 2 is: {tuple_second}");

//     let array: [i32; 5] = [1, 2, 3, 4, 5];
//     let array_second: i32 = array[2];
//     println!("The value of array at index 2 is: {array_second}");
// }

// fn variables_and_mutability() {
//     // Mutable variable.
//     let mut x: i32 = 5;
//     println!("The value of x is: {x}");

//     x = 6;
//     println!("The value of x changed to: {x}");

//     // Constant variable.
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

//     // Shadowing
//     let shadow: i32 = 10;
//     let shadow: i32 = shadow + 1;

//     println!("The value of shadow is: {shadow}");
// }
