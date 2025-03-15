use std::io;

fn main() {
    const CONV_FACTOR:f64 = 5.0/9.0;

    println!("Enter tempature in Fahrenheit.");

    let mut f_temp = String::new();

    io::stdin()
        .read_line(&mut f_temp)
        .expect("Failed to read line");
    let f_temp: f64 = match f_temp 
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => f64::NAN,
        };

    if f_temp.is_nan() {
        println!("Input not recognized as a number.");
    } else {
        let c_temp = (f_temp - 32.0) * CONV_FACTOR;
        println!("{f_temp} in Fahrenheit is {c_temp} in Celsius.");
    }
}
