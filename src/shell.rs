use std::process::Command;

#[macro_export]
macro_rules! shell {
    ($($arg:tt)*) => {{
        $crate::shell::run(format!($($arg)*))
    }};
}

pub struct ShellResult {
	pub code: i32,
	pub stdout: String,
	pub stderr: String,
}

#[allow(dead_code)]
pub fn run(command: String) -> ShellResult {
	let mut iter = command.split_whitespace();
	let mut current = iter.next();
	let mut words: Vec<&str> = vec![];
	while current != None {
		words.push(current.unwrap());
		current = iter.next();
	}
	let command = Command::new(&words[0])
		.args(&words[1..])
		.output()
		.expect(&format!("Failed to execute '{}'", command));
	return ShellResult {
		code: command.status.code().unwrap(),
		stdout: String::from_utf8(command.stdout).unwrap(),
		stderr: String::from_utf8(command.stderr).unwrap(),
	};
}
