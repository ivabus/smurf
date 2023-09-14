# SmURF

> SMall Useful Rust Functions

## Components 

### IO

- macros input!(T) and input_vec!(T), that could be used to simplify stdin process
- read_file_str() basically reads file to string (yes.)
- read_file_bytes() see read_file_str()

### Shell

- macros shell!()

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
smurf = "0.3"
```

and this to your crate root:

```rust
extern crate smurf;
```

## Example

```rust
use smurf;
use std::io::Write;

fn main() {
	print!("What is your name? ");
	std::io::stdout().flush().unwrap();
	let user_name = input!(String);
	print!("What is your age? ");
	std::io::stdout().flush().unwrap();
	let user_age = input!(u8);
	let shell_output = shell!("echo Hello {} of age {}", user_name, user_age); // Returns ShellResult {code: i32, stdout: String, stderr: String}
	println!("Shell output: {}", shell_output.stdout);
	println!("Shell stderr: {}", shell_output.stderr);
	println!("Shell exit code: {}", shell_output.code);
}
```

## License

[MIT](./LICENSE)
