extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::default::Default;

struct State {
	memory: [u8; 30000],
	instruction_pointer: usize,
	running: bool,
	stack: Vec<usize>,
}

impl Default for State {
	fn default() -> State {
		State {
			memory: [0u8; 30000],
			instruction_pointer: 0,
			running: true,
			stack: Vec::new(),
		}
	}
}

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

	// Form vector of characters based on source file,
	// and strip unnecessary characters
	let mut characters = Vec::new();
	for character in (&program_string).chars() {
		if character == '>' || character == '<' ||
			character == '+' || character == '-' ||
			character == '.' || character == ',' ||
			character == '[' || character == ']'
		{
			characters.push(character);
		}
	}

	let mut state = State::default();
	while state.running {
		match characters[state.instruction_pointer] {
			'>' => state.instruction_pointer += 1,
			'<' => state.instruction_pointer -= 1,
			'+' => state.memory[state.instruction_pointer] += 1,
			'-' => state.memory[state.instruction_pointer] -= 1,
			'[' => state.stack.push(state.instruction_pointer),
			']' => match state.stack.pop() {
				Some(value) => state.instruction_pointer = value + 1,
				None => state.running = false,
			},
			'.' => print!("{}", state.memory[state.instruction_pointer] as char),
			',' => { /* TODO: Implement user input. */ },
			_ => state.running = false,
		}

		if state.instruction_pointer >= characters.len() {
			state.running = false;
		}
	}
}
