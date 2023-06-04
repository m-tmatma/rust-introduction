use std::thread;
use std::time::Duration;

fn foo(id: i32) {
    for i in 0..10 {
        println!("thread #{} count {}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    let task = async {
        foo(10);
        foo(20);
        foo(30);
    };
    futures::executor::block_on(task);
    println!("program end.");
}
