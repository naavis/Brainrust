extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::default::Default;
use std::collections::HashMap;

struct State {
	memory: [u8; 30000],
	instruction_pointer: usize,
	data_pointer: usize,
	running: bool,
}

impl Default for State {
	fn default() -> State {
		State {
			memory: [0u8; 30000],
			instruction_pointer: 0,
			data_pointer: 0,
			running: true,
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
	let mut characters: Vec<char> = Vec::new();
	for character in (&program_string).chars() {
		if character == '>' || character == '<' ||
			character == '+' || character == '-' ||
			character == '.' || character == ',' ||
			character == '[' || character == ']'
		{
			characters.push(character);
		}
	}

	// Find matching brackets
	let mut bracket_stack = Vec::new();
	let mut matching_brackets = HashMap::new();
	for (i, character) in characters.iter().enumerate() {
		if character == &'[' {
			bracket_stack.push(i);
		} else if character == &']' {
			let matching_index = bracket_stack.pop().unwrap();
			matching_brackets.insert(i, matching_index);
			matching_brackets.insert(matching_index, i);
		}
	}

	let mut state = State::default();
	while state.running {
		match characters[state.instruction_pointer] {
			'>' => state.data_pointer += 1,
			'<' => state.data_pointer -= 1,
			'+' => state.memory[state.data_pointer] += 1,
			'-' => state.memory[state.data_pointer] -= 1,
			'[' => if state.memory[state.data_pointer] == 0 {
				state.instruction_pointer =
					matching_brackets.get(&state.instruction_pointer).unwrap().clone();
			},
			']' => if state.memory[state.data_pointer] != 0 {
				state.instruction_pointer =
					matching_brackets.get(&state.instruction_pointer).unwrap().clone();
			},
			'.' => print!("{}", state.memory[state.data_pointer] as char),
			',' => { /* TODO: Implement user input. */ },
			_ => state.running = false,
		}

		state.instruction_pointer += 1;

		if state.instruction_pointer >= characters.len() {
			state.running = false;
		}
	}
}
