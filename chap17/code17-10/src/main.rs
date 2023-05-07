fn main() {
    let format_name = |name: &str, age: i32 | {
        format!("name: {}, age: {}", name, age)
    };
    let x = format_name("kato", 40);
    println!("x is {}", x);
}
