extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn print_usage(program: &str, options: Options) {
	let brief = format!("Usage: {} [options] filename", program);
	print!("{}", options.usage(&brief));
}

fn main() {
	// Collect command line arguments
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	// Initialize command line options
	let mut options = Options::new();
	options.optflag("h", "help", "print this help menu");
	let matches = match options.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};
	if matches.opt_present("h") {
		print_usage(&program, options);
		return;
	}

	// Get filename from command line options
	let filename = if !matches.free.is_empty() {
		matches.free[0].clone()
	} else {
		print_usage(&program, options);
		return;
	};

	// Open source file
	let program_file = match File::open(filename) {
		Ok(file) => file,
		Err(..) => panic!("Could not open file!")
	};

	let mut reader = BufReader::new(&program_file);
	let program_string = &mut String::new();

	// Read Brainfuck source string from file
	let _ = reader.read_to_string(program_string);

	// Form vector of characters based on source file
	let characters: Vec<char> = (&program_string).chars().collect();

	// Loop over characters
	for character in characters {
		println!("{}", character);
	}
}
