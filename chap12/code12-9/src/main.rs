use std::fs::File;
use std::io::Read;

fn main() {
    let path = "sample.txt";
    println!("read all lines by buffer");
    if let Ok(mut file) = File::open(path) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data){
            println!("data is {}", data);
        }
    }
}
