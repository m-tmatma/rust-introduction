enum LANG {
    JAPANESE = 81,
    ENGLISH = 44,
    CHINESE = 86,
    FRENCH = 33,
}

fn main() {
    let lang = LANG::JAPANESE;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        _ => "日本語以外",
    };
    println!("lang is {}", m);
}
