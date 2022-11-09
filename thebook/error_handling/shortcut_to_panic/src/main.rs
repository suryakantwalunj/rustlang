use std::fs::File;
fn main() {
    //File::open("Hello.txt").unwrap();
    let _greeting_file = File::open("Hello.txt")
    .expect("Hello should be included into the project folder.");

}
