struct Rectangle {
    width: f32,
    height: f32,
}

struct Triangle {
    base: f32,
    height: f32,
}

struct Circle {
    radius: f32
}

fn main() {
    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let tri = Triangle {
        base: 10.0,
        height: 20.0,
    };
    let cir = Circle {
        radius: 10.0
    };
}
