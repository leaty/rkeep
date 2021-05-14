use crate::config;
use clipboard::{ClipboardContext, ClipboardProvider};
use keepass::{Database, Group, Node};
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

pub struct Session {
	pub database: Option<Database>,
	pub name: String,
	path: PathBuf,
	keyfile: Option<PathBuf>,
	alive: u32,
	clipboard: u32,
	alive_since: Option<Instant>,
	clipboard_since: Option<Instant>,
	clipped: Option<String>,
}

impl Session {
	pub fn new(config: &config::Session) -> Session {
		Session {
			database: None,
			name: config.name.clone(),
			path: config.database.clone(),
			keyfile: config.keyfile.clone(),
			alive: config.alive,
			clipboard: config.clipboard,
			alive_since: None,
			clipboard_since: None,
			clipped: None,
		}
	}

	pub fn open(&mut self, password: &String) -> Result<(), Box<dyn std::error::Error>> {
		// Open keyfile if any
		let mut key = match &self.keyfile {
			Some(k) => Some(File::open(k)?),
			_ => None,
		};

		self.database = Some(Database::open(
			&mut File::open(&self.path)?,
			Some(password),
			// keepass lib wants keyfile as an option with &mut
			match &mut key {
				Some(k) => Some(k),
				_ => None,
			},
		)?);

		println!("Opened session '{}'.", self.name);

		// Set timer
		self.alive_since = Some(Instant::now());

		Ok(())
	}

	pub fn list(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
		let mut list = vec![];
		let root = &self.database.as_ref().ok_or("Database is not open.")?.root;
		build_list(&mut list, root, "".into());
		Ok(list)
	}

	pub fn clip(&mut self, entry: &String) -> Result<(), Box<dyn std::error::Error>> {
		// Get entry
		if let Some(Node::Entry(e)) = self
			.database
			.as_ref()
			.ok_or("Database is not open.")?
			.root
			.get(&entry.split("/").collect::<Vec<&str>>())
		{
			// Clip password
			let clipped = e
				.get_password()
				.ok_or(format!("No password in entry '{}'.", entry).as_str())?
				.to_string();

			// Put password in clipboard
			let mut clipboard = ClipboardContext::new()?;
			clipboard.set_contents(clipped.clone())?;

			// Store last clipped and set timer
			self.clipped = Some(clipped);
			self.clipboard_since = Some(Instant::now());

			// Reset alive timeout
			self.alive_since = Some(Instant::now());
		} else {
			return Err(format!("Unable to fetch entry '{}'.", entry).into());
		}

		Ok(())
	}

	pub fn clip_reset(&mut self) {
		self.clipboard_since = None;
		self.clipped = None;
	}

	pub fn clean(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		// Drop database if timeout is met
		if let Some(alive) = self.alive_since {
			if alive.elapsed().as_secs() > self.alive.into() {
				self.database = None;
				self.alive_since = None;
				println!("Dropped session '{}'.", self.name);
			}
		}

		// Clear clipped if timeout is met
		if let (Some(clipped), Some(since)) = (&self.clipped, &self.clipboard_since) {
			if since.elapsed().as_secs() > self.clipboard.into() {
				let mut clipboard = ClipboardContext::new()?;

				// Clear clipboard if last clipped matches clipboard
				if clipped == &clipboard.get_contents()? {
					clipboard.set_contents("".into())?;
					println!("Cleared clipboard.");
				} else {
					println!("Clipboard was modified, skipping clear.");
				}

				// Always clear clip
				self.clip_reset();
			}
		}

		Ok(())
	}
}

fn build_list(list: &mut Vec<String>, root: &Group, parents: String) {
	// Add entries
	for (key, _) in &root.entries {
		list.push(format!("{}{}", parents, key));
	}

	// Go through children groups (recursive)
	for (_, group) in &root.child_groups {
		build_list(list, &group, format!("{}{}/", parents, group.name));
	}
}
