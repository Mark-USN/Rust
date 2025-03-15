
//let expensive_closure = |num: u32| -> u32 {
//    println!("Calculating slowly ...");
//    thread::sleep(Duration::from_secs(2));

//    num
//}

use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));

        num
    };



    println!("Try expensive_closure(5) = {}", expensive_closure(5));
}
