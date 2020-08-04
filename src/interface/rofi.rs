use rexpect::{spawn, spawn_bash};

struct CMD;
impl CMD {
	const PASSWORD: &'static str = "rofi -dmenu -i -p Password -password";
	const LIST: &'static str = "rofi -dmenu -columns 1";
}

pub fn password() -> Result<String, Box<dyn std::error::Error>> {
	let mut rofi = spawn(&CMD::PASSWORD, None)?;
	let password = rofi.read_line()?;
	Ok(password)
}

pub fn list(prompt: &String, entries: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
	let mut bash = spawn_bash(None)?;
	bash.send_line("unset HISTFILE")?;
	bash.wait_for_prompt()?;
	bash.send_line(format!("echo -e '{}' | {} -p {}", entries.join("\\n"), &CMD::LIST, prompt).as_str())?;
	let entry = bash.wait_for_prompt()?.trim().to_string();

	if entry.is_empty() {
		Err("None chosen")?
	}

	Ok(entry)
}
