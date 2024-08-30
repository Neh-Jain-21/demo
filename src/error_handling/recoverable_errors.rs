use std::env::current_dir;
use std::fs::{canonicalize, File};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

fn using_match() {
    let current_directory = canonicalize(Path::new("src/assets/Cargo.lock"));

    let mut lock_file_string: String = String::new();
    let lock_file_result: Result<File, Error> = File::open(current_directory);

    let mut lock_file: File = match lock_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(current_directory) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    lock_file.read_to_string(&mut lock_file_string);

    println!("Lock file: {}", lock_file_string);
}

pub fn recoverable_errors() {
    using_match();
}
