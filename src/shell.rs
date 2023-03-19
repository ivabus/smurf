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
		.env("PATH", "/bin:/usr/bin") // TODO: Make this configurable
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
	#[test]
	fn test_shell_macro() {
		let result = shell!("echo hello");
		assert_eq!(result.code, 0);
		assert_eq!(result.stdout, "hello\n");
		assert_eq!(result.stderr, "");
	}

	/* You literally can't use pipe using std::process::Command */
	// TODO: figure out how to test this
	#[ignore]
	#[test]
	fn test_shell_macro_with_stderr() {
		let result = shell!("echo hello 1>&2");
		assert_eq!(result.code, 0);
		assert_eq!(result.stdout, "");
		assert_eq!(result.stderr, "hello\n");
	}

	#[test]
	fn test_shell_macro_with_args() {
		let result = shell!("echo {} {}", "hello", "world");
		assert_eq!(result.code, 0);
		assert_eq!(result.stdout, "hello world\n");
		assert_eq!(result.stderr, "");
	}

	// TODO: figure out how to test this
	#[ignore]
	#[test]
	fn test_shell_macro_with_code() {
		let result = shell!("sh -c exit 1");
		assert_eq!(result.code, 1);
		assert_eq!(result.stdout, "");
		assert_eq!(result.stderr, "");
	}

}