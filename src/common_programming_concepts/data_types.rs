pub fn data_types() {
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
