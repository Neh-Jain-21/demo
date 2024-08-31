use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};

fn using_match() {
    let mut lock_file_string: String = String::new();
    let current_directory: &str = "src/assets/text.txt";
    let lock_file_result: Result<File, Error> = File::open(&current_directory);

    let mut lock_file: File = match lock_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&current_directory)
            {
                Ok(mut fc) => match fc.write_all("Hello Lock".as_bytes()) {
                    Ok(_) => fc,
                    Err(e) => panic!("Problem writing to the file: {e:?}"),
                },
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    lock_file.read_to_string(&mut lock_file_string).unwrap();

    println!("Lock file: {}", lock_file_string);
}

fn chaining() -> Result<String, Error> {
    let mut lock_file_string: String = String::new();
    let current_directory: &str = "src/assets/text.txt";

    let _ = File::open(&current_directory)?.read_to_string(&mut lock_file_string);

    Ok(lock_file_string)
}

pub fn recoverable_errors() {
    using_match();
    let _ = chaining();
}
