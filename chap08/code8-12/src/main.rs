fn main() {
    struct Color(f32,f32,f32);
    let yellow = Color(1.0, 1.0, 1.0);
    println!("R:{:2} G:{:2} B:{:2}", yellow.0, yellow.1, yellow.2);
}
