fn main() {
    let s = "one,two,three,four,five";
    let v = s.split(',');
    for x in v {
        println!("{} ", x);
    }
    println!("");
}
