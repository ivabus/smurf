use log::error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_to_str(path: &std::path::PathBuf) -> String {
	let file = File::open(&path);
	let file = match file {
		Ok(data) => data,
		Err(err) => {
			error!("Cannot open file \"{}\"", path.display());
			std::process::exit(2)
		}
	};
	let mut buf_reader = BufReader::new(file);
	let mut content = String::new();
	buf_reader.read_to_string(&mut content).unwrap();
	content
}