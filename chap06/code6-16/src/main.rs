fn main() {
    print!("FOR is ");
    for i in 0..10 {
        if i == 5 {
            break;
        }
        print!("{} ", i);
    }
    println!("");
}
