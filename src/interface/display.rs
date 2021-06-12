use std::io::Write;
use std::process::{Command, Stdio};

pub fn password(cmd: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
	let output = Command::new(&cmd[0]).args(&cmd[1..]).output()?;
	let password = String::from_utf8(output.stdout)?.trim().to_string();
	Ok(password)
}

pub fn list(
	cmd: &Vec<String>,
	entries: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
	let mut child = Command::new(&cmd[0])
		.args(&cmd[1..])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	{
		let stdin = child.stdin.as_mut().ok_or("Failed to open stdin.")?;
		stdin.write_all(entries.join("\n").as_bytes())?;
	}

	let output = child.wait_with_output()?;
	let entry = String::from_utf8(output.stdout)?.trim().to_string();
	if entry.is_empty() {
		Err("None chosen.")?
	}

	Ok(entry)
}
