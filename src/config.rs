use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	pub session: Vec<Session>,
	pub socket: PathBuf,
}

#[derive(Deserialize)]
pub struct Session {
	pub name: String,
	pub database: PathBuf,
	pub alive: u32,
	pub timeout: u64,
	pub clipboard: u32,
}

