fn main() {
    let v = vec![1,2,3,4,5];
    println!("v[0] is {:?}", v.get(0));
    println!("v[4] is {:?}", v.get(4));
    println!("v[0] is {:?}", v.get(0).unwrap());
    println!("v[4] is {:?}", v.get(4).unwrap());
    println!("v.len is {}", v.len());
}
