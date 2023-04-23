fn half_number(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(_err) => Err("実行エラー: これは数値ではありません"),
    }
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
