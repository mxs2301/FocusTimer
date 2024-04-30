use std::thread;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    println!("Hello, world!");
    let handle = thread::spawn(|| {
        wait("A");
    });
    let second = thread::spawn(|| {
        wait("B");
    });

    second.join().unwrap();
}

fn wait<T: std::fmt::Display>(x: T) {
    let y = Duration::from_secs(2);
    loop {
        println!("{}", x);
        sleep(y);
    }
}
