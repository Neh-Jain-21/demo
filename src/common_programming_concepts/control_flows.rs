fn if_else_fn() {
    let number: i32 = 5;

    if number == 5 {
        println!("Number is 5");
    } else if number >= 0 {
        print!("Number is positive")
    } else {
        println!("Number is negative");
    }
}

fn short_hand_if_else_fn() {
    let condition: bool = true;
    let result: i32 = if condition { 1 } else { 0 };

    println!("The number is {result}");
}

fn loop_fn() {
    let mut counter: i32 = 0;

    let counter_result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Counter value is {counter_result}");
}

fn while_fn() {
    let mut counter: i32 = 10;

    while counter != 0 {
        println!("{counter}");

        counter -= 1;
    }

    println!("While loop over!");
}

fn for_fn() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a {
        println!("For loop count {element}");
    }

    println!("For loop over");
}

pub fn control_flows() {
    if_else_fn();
    short_hand_if_else_fn();
    loop_fn();
    while_fn();
    for_fn();
}
