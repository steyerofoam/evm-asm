use getopts::Options;
use std::env;
use std::fs;
use std::process;

mod tokenizer;

fn print_usage(pname: &str, opts: Options) {
	let brief = format!("Usage: {} [options] [FILE]", pname);
	print!("{}", opts.usage(&brief));
}

fn main() {
	// args
	let args: Vec<String> = env::args().collect();
	let pname = args[0].clone();

	// setup options
	let mut opts = Options::new();

	opts.optflag("h", "help", "Prints this help menu.");

	// parse options
	let mut matches = match opts.parse(&args[1..]) {
		Ok(opt) => {opt}
		Err(e) => {
			eprintln!("{}.", e);
			print_usage(&pname, opts);
			process::exit(exitcode::USAGE);
		}
	};

	// validate and/or execute options
	if matches.opt_present("h") {
		print_usage(&pname, opts);
		return;
	}

	if matches.free.is_empty() {
		eprintln!("Must pass file to assemble.");
		process::exit(exitcode::USAGE);
	} else {
		let filename = matches.free.remove(0);
		let Ok(input) = fs::read_to_string(&filename) else {
			eprintln!("File cannot be read: {}", filename);
			process::exit(exitcode::NOINPUT);
		};

		let tokenize_result = tokenizer::tokenize(&input, &filename);
		let Ok(tokens) = tokenize_result else {
			eprintln!("Tokenizer error: {}", tokenize_result.err().unwrap());
			process::exit(exitcode::DATAERR);
		};

		for token in tokens {
			println!("{token}");
		}
	}
}