struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn print_person (pa: &Person) {
    println!("{}: {} ({}) in {}",
        pa.id, pa.name, pa.age, pa.addr
    );
}

fn main() {
    let mut pa = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.age += 1;
    print_person(&pa);
    pa.addr = String::from("Osaka");
    print_person(&pa);
}
