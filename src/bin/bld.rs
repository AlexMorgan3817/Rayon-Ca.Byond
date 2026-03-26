use std::io::{Read, stdin};
use std::process::Command;

fn main() {
	let mut input = stdin();
	println!("Your target [w]indows or [l]inux or [a]ll?");
	let mut buf: [u8; 1] = [0];
	let eof = input.read(&mut buf);
	if eof.is_err() {
		return;
	}
	let arch = buf.to_ascii_lowercase();
	let letter = arch[0] as char;
	let arch: Vec<&str> = match letter {
		'w' => ["i686-pc-windows-msvc"].to_vec(),
		'l' => ["i686-unknown-linux-gnu"].to_vec(),
		'a' => ["i686-pc-windows-msvc", "i686-unknown-linux-gnu"].to_vec(),
		_ => {
			println!("Invalid input");
			return;
		}
	};
	for target in arch {
		println!("Building for {}", target);
		match Command::new("cargo")
			.arg("build")
			.arg("--release")
			.args(["--target", target])
			.status()
		{
			Ok(status) => {
				if !status.success() {
					match status.code() {
						Some(code) => {
							println!("Failed to build for {}: {}", target, code);
							match code {
								101 => {
									println!("CC linker is missing, you should install it or better build directly on Linux.");
								}
								_ => {}
							}
						}
						None => {
							println!("Failed to build for {}: ", target);
						}
					}
				}
			}
			Err(e) => {
				println!("Failed to start cargo for {}: {}", target, e);
			}
		}
	}
}
