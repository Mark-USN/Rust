use std::sync::mpsc;
use std::thread;

fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let val = String::from("hi");
		tx.send(val).unwrap();
		println!("val is {val}");
	});

	let recieved = rx.recv().unwrap();
	println!("Got: {recieved}");

}