#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}
fn print_a(a: &Person ){
    println!("print_a: a is {:?}", a)
}

fn main() {
    let a = Person { name: "masuda", age: 50};
    let x = &a;
    print_a(&a);
    print_a(&x);
}
