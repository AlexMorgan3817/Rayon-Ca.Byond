use std::process::Command;
use std::fs;

fn main() {
	println!("Starting cargo build.");
	let status = Command::new("cargo")
		.args([
			"build",
			"--target",
			if cfg!(target_os = "windows")
			{ "i686-pc-windows-msvc" }
			else
			{ "i686-unknown-linux-gnu" },
		])
		.status()
		.expect("failed to build");
	if !status.success(){
		println!("Can't build a lib: {}", status.code().unwrap());
		return;
	}
	let (from, to) = if cfg!(target_os = "windows") {
		("target\\i686-pc-windows-msvc\\debug\\rayon_ca_byond.dll", "test\\lib.dll")
	} else {
		("target/i686-unknown-linux-gnu/debug/librayon_ca_byond.so", "target/lib.so")
	};
	fs::copy(from, to).expect("failed to copy");
	let dmargs: Vec<&str> = if cfg!(target_os = "windows") {
		vec!["-DVERBOSE", "test/test.dme"]
	} else {
		vec!["-DLINUX", "-DVERBOSE", "test/test.dme"]
	};
	let dm = Command::new(if cfg!(target_os = "windows") {"dm.exe"} else { "dm" })
		.args(dmargs)
		.status()
		.expect("failed to run dm");
	if dm.success() {
		let dd = Command::new("dd.exe")
			.args([
				"test/test.dmb",
				"-trusted"
			])
			.status()
			.expect("failed to run dd");
		if dd.success() {
			println!("Run successful");
		}
		else {
			println!("ERR: There was an error in running: {}", match dd.code() {
				Some(code) => code.to_string(),
				None => "NO ERROR CODE".to_string(),
			});
		}
	}
	else{
		println!("ERR: There was an error in compilation: {}", match dm.code() {
			Some(code) => code.to_string(),
			None => "NO ERROR CODE".to_string(),
		});
	}
}
