use  std::fs::File;
use std::io::ErrorKind;
#[allow(unused_variables )]
fn main() {
    let greeting_file_result = File::open("Hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem with creating file {:?}",e),
            },
            other_error => {
                panic!("Problem with opening file {:?}",other_error);
            }
        }
        ,
    };
    //println!("greeting file {}",greeting_file);
}
