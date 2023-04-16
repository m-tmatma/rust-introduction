fn main() {
    let x = 5;
    let m = match x {
        0..=5 => "5以下",
        6..=10 => "10以下",
        _ => "10より大きい",
    };

    println!("m is {}", m);
}
