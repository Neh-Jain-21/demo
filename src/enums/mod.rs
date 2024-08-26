pub mod enums {
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

    pub fn enums() {
        Message::Text(String::from("Hello World")).call();
        Message::Color(0, 0, 0).call();
    }
}
