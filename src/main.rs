fn main() {
    // variables_and_mutability();
    // data_types();
    // functions();
    // control_flows();

    ownership();
}

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

// fn five() -> i32 {
//     return 5;
// }

// fn functions() {
//     let x: i32 = five();
//     println!("The value of x is: {x}");
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

fn ownership() {
    let s: String = String::from("hello");

    takes_ownership(s);

    let x: i32 = 5;

    makes_copy(x);

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }
}
