#![allow(unused_variables)]
use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let greeting_file_result = File::open("hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem on creating a file {:?}",e)
            }
            other_error => {
                panic!("Could not open the file. {:?}",other_error);
            }
        },
    };
    
}