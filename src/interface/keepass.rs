use crate::config;
use std::path::PathBuf;
use rexpect::spawn_bash;
use rexpect::session::PtyReplSession;

struct CMD;
impl CMD {
	const OPEN: &'static str = "keepassxc-cli open";
	const CLIP: &'static str = "clip";
}

pub struct Session {
	pub connection: Option<PtyReplSession>,
	database: PathBuf,
	alive: u32,
	timeout: u64,
	clipboard: u32,
}

impl Session {
	pub fn new(config: &config::Session) -> Session {
		Session {
			connection: None,
			database: config.database.clone(),
			alive: config.alive,
			timeout: config.timeout,
			clipboard: config.clipboard,
		}
	}

	pub fn open(&mut self, password: &String) -> Result<(), Box<dyn std::error::Error>> {
		let mut keep = spawn_bash(Some(self.timeout))?;
		keep.send_line("unset HISTFILE")?;
		keep.wait_for_prompt()?;
		keep.execute(format!("timeout {} {} {}", self.alive, CMD::OPEN, self.database.display()).as_str(), "Enter password to unlock")?;
		keep.send_line(&password)?;
		keep.exp_regex(".kdbx>")?;
		self.connection = Some(keep);
		Ok(())
	}

	pub fn list(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
		// Send ls command but ignore output of the command itself
		self.connection.as_mut().unwrap().send_line("ls -fR")?;

		// Read output and collect
		let listing = self.connection.as_mut().unwrap().exp_regex(".kdbx>")?.0;
		let mut entries: Vec<String> = listing.lines().map(|s| s.to_string()).collect();

		// Remove ls command, prompt line and all entries ending with /
		entries.remove(0);
		entries.pop();
		entries.retain(|x| x.chars().last().unwrap() != '/');

		Ok(entries)
	}

	pub fn clip(&mut self, entry: &String) -> Result<(), Box<dyn std::error::Error>> {
		self.connection.as_mut().unwrap().send_line(format!("{} {}", &CMD::CLIP, &entry).as_str())?;
		self.connection.as_mut().unwrap().read_line()?;
		self.connection.as_mut().unwrap().exp_regex(".kdbx>")?;
		Ok(())
	}
}

