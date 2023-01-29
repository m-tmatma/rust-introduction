#[derive(Debug)]
enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRENCH,
}

fn main() {
    let lang = LANG::JAPANESE;
    println!("lang is {:?}", lang);
}
