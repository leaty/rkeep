use std::process::Command;

struct CMD;
impl CMD {
	const PASSWORD: &'static str =
		"rofi -dmenu -i -p Password -password -theme-str 'entry { placeholder: \"\"; }'";
	const LIST: &'static str = "rofi -no-auto-select -dmenu -columns 1";
}

pub fn password() -> Result<String, Box<dyn std::error::Error>> {
	let output = Command::new("sh").args(&["-c", &CMD::PASSWORD]).output()?;
	let password = String::from_utf8(output.stdout)?.trim().to_string();
	Ok(password)
}

pub fn list(prompt: &String, entries: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
	let output = Command::new("sh")
		.arg("-c")
		.arg(format!(
			"echo -e '{}' | {} -p {}",
			entries.join("\\n"),
			CMD::LIST,
			prompt,
		))
		.output()?;

	let entry = String::from_utf8(output.stdout)?.trim().to_string();

	if entry.is_empty() {
		Err("None chosen.")?
	}

	Ok(entry)
}
