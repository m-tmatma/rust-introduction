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

fn new_person (id: i32, name: &str) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };

    // return explicitly
    return pa;
}

fn main() {
    let pa = new_person(100, "masuda");
    let pa2 = new_person(200, "kato");
    let mut people = vec![pa, pa2];
    people.push(new_person(200, "yamada"));
    people.push(new_person(200, "sato"));

    for p in &people {
        print_person(p);
    }
}
