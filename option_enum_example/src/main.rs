fn main() {
    let mut numbers = Vec::new();
    numbers.push(1);
    //numbers.push(2);

    let value = numbers.pop();
    if let Some(val) = value {
        println!("value {}", val)
    }

    let value = numbers.pop();
    match value {
        None => println!("Null value.."),
        Some(val) => println!("value {}", val),
    }

}
