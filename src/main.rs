use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match read_username_from_file() {
        Err(e) => println!("{}", e),
        Ok(s) => println!("{}", s)
    };
}
