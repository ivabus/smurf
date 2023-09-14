use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file_str(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
	let file = File::open(&path)?;
	let mut buf_reader = BufReader::new(file);
	let mut content = String::new();
	buf_reader.read_to_string(&mut content)?;
	Ok(content)
}

pub fn read_file_bytes(path: &std::path::PathBuf) -> Result<Vec<u8>, std::io::Error> {
	let file = File::open(&path)?;
	let mut buf_reader = BufReader::new(file);
	let mut content: Vec<u8> = Vec::new();
	buf_reader.read_to_end(&mut content)?;
	Ok(content)
}

#[macro_export]
macro_rules! input {
	($t:ty) => {{
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		input.trim().parse::<$t>().unwrap()
	}};
}

#[macro_export]
macro_rules! input_vec {
	($t:ty) => {{
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		input.trim().split_whitespace().map(|x| x.parse::<$t>().unwrap()).collect::<Vec<$t>>()
	}};
}
