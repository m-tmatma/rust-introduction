fn main() {
    println!("test = {}", test(1));
}

fn test(x: i32) -> i32 {
    let ans = if x < 0 {
        0
    } else if x > 100 {
        100
    }
    else {
        x
    };
    ans
}
