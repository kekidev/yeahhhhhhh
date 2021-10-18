use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
    let f = File::open("hello.txt");

    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

}