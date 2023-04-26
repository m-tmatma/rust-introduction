#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

fn main() {
    let a = Person { name: "masuda", age: 50};
    let x = a;
    let y = a;
    println!("a is {:?}", a);
    println!("x is {:?}", x);
    println!("y is {:?}", y);
}
