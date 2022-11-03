fn main() {
    fn divide(numerator : i64, denometer : i64) -> Option<i64> {
        if denometer == 0 {
            None
        } else {
            Some(numerator/denometer)
        }
    }
    //Return value of function is Result
    let result = divide(10, 0);
    //Pattern match to retrive value.
    match result {
        None => println!("Divide by zero."),
        Some(val) => println!("value {}",val),
    }
}
