fn get_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

pub fn generic_types() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let result: &i32 = get_largest(&number_list);

    println!("{result}");

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];

    let result: &char = get_largest(&char_list);

    println!("{result}");
}
