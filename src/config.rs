use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
	pub socket: PathBuf,
	pub session: Vec<Session>,
}

#[derive(Deserialize, Clone)]
pub struct Session {
	pub name: String,
	pub database: PathBuf,
	pub keyfile: Option<PathBuf>,
	pub alive: u32,
	pub clipboard: u32,
	pub command: Command,
}

#[derive(Deserialize, Clone)]
pub struct Command {
	pub pass: Vec<String>,
	pub list: Vec<String>,
}

impl Session {
	/// Parses special values in command, just add more here if needed.
	/// TODO: Avoid WET
	pub fn parse(&mut self) {
		for arg in &mut self.command.pass {
			*arg = arg.replace("{session.name}", &self.name);
		}
		for arg in &mut self.command.list {
			*arg = arg.replace("{session.name}", &self.name);
		}
	}
}
