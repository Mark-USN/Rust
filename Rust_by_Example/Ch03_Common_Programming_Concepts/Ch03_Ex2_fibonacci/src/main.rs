use std::io;
use std::io::Write;

fn main() {

    const FIB_0:u64 = 0;
    const FIB_1:u64 = 1;
    
    let mut fib_iteration = String::new();

    print!("Enter the position in the sequence of the Fibonacci numbers to calculate: ");
    io::stdout()
        .flush()
        .expect("Failed to flush standard out!");

    io::stdin()
        .read_line(&mut fib_iteration)
        .expect("Failed to read line");

    let fib_iteration:u64 = fib_iteration
        .trim()
        .parse()
        .expect("Input not recognised as an unsigned integer.");

    let mut fib_prev:u64 = FIB_0;
    let mut fib_current:u64 = FIB_1;
    let mut fib_temp:u64; // = FIB_0;

    if fib_iteration == 0 {
        println!("The value of Fibonacci number {fib_iteration} is {FIB_0}");
    } else {
        for _count in 1..fib_iteration {
            fib_temp = fib_current + fib_prev;
            fib_prev = fib_current;
            fib_current = fib_temp;
        }
        println!("The value of Fibonacci number {fib_iteration} is {fib_current}");
    }


}
