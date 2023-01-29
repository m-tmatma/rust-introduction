enum LANG {
    JAPANESE = 81,
    ENGLISH = 44,
    CHINESE = 86,
    FRENCH = 33,
}

fn main() {
    let lang = LANG::JAPANESE;
    println!("lang is {}", lang as i32);
}
