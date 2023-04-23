use std::num::ParseIntError;
type Result<T> = std::result::Result<T, ParseIntError>;

fn half_number(s: &str) -> Result<i32> {
    let n = s.parse::<i32>()?;
    Ok(n / 2)
}

fn main() {
    match half_number("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number("xxx") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
