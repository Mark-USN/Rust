use std::env;
// use std::fs;
use std::process;
// use std::error::Error;
use minigrep::Config;


fn main() {
	let args: Vec<String> = env::args().collect();
	
	// let query = &args[1];
	// let file_path = &args[2];
	// let (query, file_path) = parse_config(&args);
	// let config = parse_config(&args);
	// let config = Config::new(&args);
	let config = Config::build(&args).unwrap_or_else(|err| {
			eprintln!("Problem parsing arguments: {err}");
			process::exit(1);
		});

	// println!("Searching for {}", query);
	// println!("In file {}", file_path);
	// println!("Searching for `{}`", config.query);
	// println!("In file {}", config.file_path);

	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {e}");
		process::exit(1);
	};

	//let contents = fs::read_to_string(config.file_path)
 //   .expect("Should have been able to read the file.");

	//println!("With text:\n{contents}");
}

//fn run(config: Config) -> Result<(), Box<dyn Error>> {

//	let contents = fs::read_to_string(config.file_path) ?;
//    // .expect("Should have been able to read the file.");

//	println!("With text:\n{contents}");

//	Ok(())
//}


//struct Config {
//	query: String,
//	file_path: String,
//}

////fn parse_config(args: &[String]) -> (&str, &str) {
////	let query = &args[1];
////	let file_path = &args[2];

////	(query, file_path)
////}

////fn parse_config(args: &[String]) -> Config {
////	let query = args[1].clone();
////	let file_path = args[2].clone();

////	Config {query, file_path}
////}

//impl Config {
//	//fn new(args: &[String]) -> Config {
//	//	if args.len() < 3 {
//	//		panic!("Not enough arguments");
//	//	}
//	//	let query = args[1].clone();
//	//	let file_path = args[2].clone();

//	//	Config {query, file_path}
//	//}

//	fn build(args: &[String]) -> Result<Config, &'static str> {
//		if args.len() < 3 {
//			return Err("Not enough arguments:\n\tminigrep SearchString File\n");
//		}
//		let query = args[1].clone();
//		let file_path = args[2].clone();

//		Ok(Config {query, file_path})
//	}

//}
