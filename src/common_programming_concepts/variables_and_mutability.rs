pub fn variables_and_mutability() {
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
