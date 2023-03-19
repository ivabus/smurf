use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file_to_str(path: &std::path::PathBuf) -> Option<String> {
	let file = File::open(&path);
	let file = match file {
		Ok(data) => data,
		Err(_) => {
			eprintln!("Cannot open file \"{}\"", path.display());
			return None
		}
	};
	let mut buf_reader = BufReader::new(file);
	let mut content = String::new();
	buf_reader.read_to_string(&mut content).unwrap();
	Some(content)
}


#[macro_export]
macro_rules! input {
	($t:ty) => {
		{
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).unwrap();
			input.trim().parse::<$t>().unwrap()
		}
	}
}

#[macro_export]
macro_rules! input_vec {
	($t:ty) => {
		{
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).unwrap();
			input.trim().split_whitespace().map(|x| x.parse::<$t>().unwrap()).collect::<Vec<$t>>()
		}
	}
}
