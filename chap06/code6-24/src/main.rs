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
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        LANG::FRENCH => "フランス語",
    };
    println!("lang is {}", m);
}
