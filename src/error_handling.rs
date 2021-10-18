use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub fn run() {
    // let f = File::open("hello.txt");

    /* this code equals to below
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
    }; */


    /*
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hellot.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file! {:?}", error);
            })
        } else {
            panic!("Problem opening file! {:?}", error);
        }
    }); */

    //let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("Failed to open file");
    let d = read_username_from_file();
    
}

/* this code equals to
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
} */
// to this
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
