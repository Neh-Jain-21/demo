struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn struct_method_syntax() {
    let rectangle1: Rectangle = Rectangle {
        width: 20,
        height: 20,
    };

    let rectangle2: Rectangle = Rectangle {
        width: 30,
        height: 30,
    };

    println!("Area of Rectangle 1: {}", rectangle1.area());
    println!("Area of Rectangle 2: {}", rectangle2.area());

    println!("Can 1 hold 2: {}", rectangle1.can_hold(&rectangle2));
    println!("Can 2 hold 1: {}", rectangle2.can_hold(&rectangle1));
}
