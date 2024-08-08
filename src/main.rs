fn main() {
    println!("Hello, world!");

    variables_and_mutability();
    data_types();
    functions();
}

fn variables_and_mutability() {
    // Mutable variable.
    let mut x: i32 = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x changed to: {x}");

    // Constant variable.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let shadow: i32 = 10;
    let shadow: i32 = shadow + 1;

    println!("The value of shadow is: {shadow}");
}

fn data_types() {
    const _SIGNED_INTEGER: usize = 5;
    let _unsigned_integer: i32 = -10;
    let _floating_point: f64 = 10.0;
    let _character: char = 'a';
    let _boolean: bool = true;
    let _string: &str = "Hello, world!";

    let tuple: (i32, &str) = (10, "Hello, world!");
    let tuple_second: &str = tuple.1;
    println!("The value of tuple at index 2 is: {tuple_second}");

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_second: i32 = array[2];
    println!("The value of array at index 2 is: {array_second}");
}

fn five() -> i32 {
    return 5;
}

fn functions() {
    let x: i32 = five();
    println!("The value of x is: {x}");
}
