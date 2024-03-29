struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

impl Person {
    fn print(&self) {
        println!("{}: {} ({}) in {}",
            self.id, self.name, self.age, self.addr);
    }
}

fn main() {
    let pa =Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo",)
    };
    pa.print();
}
