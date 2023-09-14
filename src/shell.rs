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
fn run(command: String) -> ShellResult {
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

#[cfg(test)]
mod tests {
	// Testing basic usage and only on unix

	#[cfg(unix)]
	#[test]
	fn test_shell_macro() {
		let result = shell!("echo hello");
		assert_eq!(result.code, 0);
		assert_eq!(result.stdout, "hello\n");
		assert_eq!(result.stderr, "");
	}

	#[cfg(unix)]
	#[test]
	fn test_shell_macro_with_args() {
		let result = shell!("echo {} {}", "hello", "world");
		assert_eq!(result.code, 0);
		assert_eq!(result.stdout, "hello world\n");
		assert_eq!(result.stderr, "");
	}
}
